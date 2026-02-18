//! Procedural macros that turn async Rust functions into
//! [Vercel](https://vercel.com) serverless lambda handlers.
//!
//! Part of the [`vercel-rpc`](https://github.com/misha-mad/vercel-rpc) project
//! — end-to-end typesafe RPC between Rust lambdas on Vercel and any TypeScript frontend.
//!
//! # Quick Start
//!
//! Add the macro crate **and** its runtime dependencies to your `Cargo.toml`
//! (the generated code uses these crates directly, so they must be present):
//!
//! ```toml
//! [dependencies]
//! vercel-rpc-macro = "0.1"
//! vercel_runtime   = "1"
//! serde            = { version = "1", features = ["derive"] }
//! serde_json       = "1"
//! tokio            = { version = "1", features = ["macros"] }
//! url              = "2"
//! http-body-util   = "0.1"
//! ```
//!
//! Then annotate an async function:
//!
//! ```rust,ignore
//! use vercel_rpc_macro::rpc_query;
//!
//! #[rpc_query]
//! async fn hello(name: String) -> String {
//!     format!("Hello, {}!", name)
//! }
//! ```
//!
//! This single attribute generates a complete Vercel-compatible binary with a
//! `main()` entry point — no extra boilerplate needed.
//!
//! # What Gets Generated
//!
//! Each macro invocation produces:
//!
//! - A `#[tokio::main]` entry point that calls `vercel_runtime::run`.
//! - An `OPTIONS` handler that returns `204` with CORS headers.
//! - HTTP method validation (`GET` for queries, `POST` for mutations).
//! - Input deserialization — from the `?input=<JSON>` query parameter (queries)
//!   or from the JSON request body (mutations).
//! - Serialization of the return value into a JSON response.
//! - Automatic error responses when the function returns `Result::Err`.
//!
//! # Response Format
//!
//! **Success** (HTTP 200):
//! ```json
//! { "result": { "type": "response", "data": <value> } }
//! ```
//!
//! **Error** (HTTP 400):
//! ```json
//! { "error": { "type": "error", "message": "<description>" } }
//! ```
//!
//! # Supported Signatures
//!
//! | Signature | Input | Output |
//! |-----------|-------|--------|
//! | `async fn f() -> T` | none | `T` serialized |
//! | `async fn f(input: I) -> T` | `I` deserialized | `T` serialized |
//! | `async fn f() -> Result<T, E>` | none | `Ok` → 200, `Err` → 400 |
//! | `async fn f(input: I) -> Result<T, E>` | `I` deserialized | `Ok` → 200, `Err` → 400 |
//!
//! More than one parameter is a **compile error**.
//!
//! # CORS
//!
//! Every response includes the following headers:
//!
//! - `Access-Control-Allow-Origin: *`
//! - `Access-Control-Allow-Methods: GET, POST, OPTIONS`
//! - `Access-Control-Allow-Headers: Content-Type`
//! - `Access-Control-Max-Age: 86400`
//!
//! # Companion Crate
//!
//! [`vercel-rpc-cli`](https://crates.io/crates/vercel-rpc-cli) scans your
//! `#[rpc_query]` / `#[rpc_mutation]` functions and generates TypeScript type
//! definitions and a fully typed RPC client for use in any TypeScript frontend.

use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, ReturnType, Type, parse_macro_input};

/// Generates a Vercel-compatible lambda handler from an async **query** function.
///
/// The annotated function becomes a **GET** endpoint. Input is read from the
/// `?input=<JSON>` query parameter and the return value is serialized as JSON.
///
/// The macro generates:
/// - A `#[tokio::main]` entry point that calls `vercel_runtime::run`.
/// - Automatic JSON deserialization of the `input` query parameter.
/// - Automatic JSON serialization of the return value.
/// - CORS headers on every response (including `OPTIONS` preflight → `204`).
/// - HTTP method validation — only `GET` is accepted; other methods return `405`.
/// - If the function returns `Result<T, E>`, `Err` is mapped to a `400` JSON
///   error response automatically.
///
/// # Examples
///
/// **No input:**
///
/// ```rust,ignore
/// use vercel_rpc_macro::rpc_query;
///
/// #[rpc_query]
/// async fn version() -> String {
///     "1.0.0".to_string()
/// }
/// ```
///
/// **With input parameter:**
///
/// ```rust,ignore
/// use vercel_rpc_macro::rpc_query;
///
/// #[rpc_query]
/// async fn hello(name: String) -> String {
///     format!("Hello, {}!", name)
/// }
/// ```
///
/// **Returning `Result`:**
///
/// ```rust,ignore
/// use vercel_rpc_macro::rpc_query;
///
/// #[rpc_query]
/// async fn find_user(id: u32) -> Result<String, String> {
///     if id == 0 {
///         Err("not found".into())
///     } else {
///         Ok(format!("user_{}", id))
///     }
/// }
/// ```
///
/// **Returning a `Vec`:**
///
/// ```rust,ignore
/// use vercel_rpc_macro::rpc_query;
///
/// #[rpc_query]
/// async fn list_tags() -> Vec<String> {
///     vec!["rust".into(), "vercel".into()]
/// }
/// ```
///
/// # Compile errors
///
/// The macro rejects functions with more than one parameter:
///
/// ```rust,compile_fail,ignore
/// #[rpc_query]
/// async fn bad(a: String, b: u32) -> String { todo!() }
/// // error: RPC handlers accept at most one input parameter
/// ```
#[proc_macro_attribute]
pub fn rpc_query(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "rpc_query does not accept any arguments",
        )
        .to_compile_error()
        .into();
    }
    let input_fn = parse_macro_input!(item as ItemFn);
    build_handler(input_fn, HandlerKind::Query)
        .map(Into::into)
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Generates a Vercel-compatible lambda handler from an async **mutation** function.
///
/// Works like [`rpc_query`] but creates a **POST** endpoint.
/// Input is read from the **JSON request body** instead of query parameters.
///
/// The macro generates:
/// - A `#[tokio::main]` entry point that calls `vercel_runtime::run`.
/// - Automatic JSON deserialization of the request body.
/// - Automatic JSON serialization of the return value.
/// - CORS headers on every response (including `OPTIONS` preflight → `204`).
/// - HTTP method validation — only `POST` is accepted; other methods return `405`.
/// - If the function returns `Result<T, E>`, `Err` is mapped to a `400` JSON
///   error response automatically.
///
/// # Examples
///
/// **No input:**
///
/// ```rust,ignore
/// use vercel_rpc_macro::rpc_mutation;
///
/// #[rpc_mutation]
/// async fn reset_counter() -> u32 {
///     0
/// }
/// ```
///
/// **With a struct input:**
///
/// ```rust,ignore
/// use vercel_rpc_macro::rpc_mutation;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct CreateUserInput {
///     name: String,
///     email: String,
/// }
///
/// #[rpc_mutation]
/// async fn create_user(input: CreateUserInput) -> String {
///     format!("Created {}", input.name)
/// }
/// ```
///
/// **Returning `Result`:**
///
/// ```rust,ignore
/// use vercel_rpc_macro::rpc_mutation;
///
/// #[rpc_mutation]
/// async fn delete_user(id: u32) -> Result<String, String> {
///     if id == 0 {
///         Err("cannot delete root user".into())
///     } else {
///         Ok(format!("deleted user {}", id))
///     }
/// }
/// ```
///
/// # Compile errors
///
/// The macro rejects functions with more than one parameter:
///
/// ```rust,compile_fail,ignore
/// #[rpc_mutation]
/// async fn bad(a: String, b: u32) -> String { todo!() }
/// // error: RPC handlers accept at most one input parameter
/// ```
#[proc_macro_attribute]
pub fn rpc_mutation(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "rpc_mutation does not accept any arguments",
        )
        .to_compile_error()
        .into();
    }
    let input_fn = parse_macro_input!(item as ItemFn);
    build_handler(input_fn, HandlerKind::Mutation)
        .map(Into::into)
        .unwrap_or_else(|e| e.to_compile_error().into())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HandlerKind {
    Query,
    Mutation,
}

/// Transforms a user-defined async function into a complete Vercel lambda handler.
///
/// Generates `main()`, CORS helpers, input parsing, and response serialization.
/// The `kind` parameter determines whether the handler accepts GET (query) or POST (mutation).
fn build_handler(func: ItemFn, kind: HandlerKind) -> Result<proc_macro2::TokenStream, syn::Error> {
    if func.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            func.sig.fn_token,
            "RPC handlers must be async functions",
        ));
    }

    let fn_name = &func.sig.ident;
    let fn_block = &func.block;
    let fn_output = &func.sig.output;

    let mut params = func.sig.inputs.iter().filter_map(|arg| match arg {
        FnArg::Typed(pat) => Some(pat),
        _ => None,
    });

    let first_param = params.next();

    // Validate: at most one parameter allowed
    if params.next().is_some() {
        return Err(syn::Error::new_spanned(
            &func.sig.inputs,
            "RPC handlers accept at most one input parameter",
        ));
    }

    let input_type = if let Some(param) = first_param {
        let ty = &param.ty;
        quote! { #ty }
    } else {
        quote! { () }
    };

    let input_pat = if let Some(param) = first_param {
        let pat = &param.pat;
        quote! { #pat }
    } else {
        quote! { _ }
    };

    // Determine whether the handler returns Result<T, E> or plain T
    let (return_type, returns_result) = match fn_output {
        ReturnType::Default => (quote! { () }, false),
        ReturnType::Type(_, ty) => (quote! { #ty }, is_result_type(ty)),
    };

    let expected_method = match kind {
        HandlerKind::Query => "GET",
        HandlerKind::Mutation => "POST",
    };

    let parse_input = match kind {
        HandlerKind::Query => quote! {
            let __input: #input_type = {
                let __url = match ::url::Url::parse(
                    &format!("http://localhost{}", __req.uri())
                ) {
                    Ok(u) => u,
                    Err(e) => return __rpc_error_response(400, &format!("Invalid URL: {}", e)),
                };

                let __raw = __url
                    .query_pairs()
                    .find(|(k, _)| k == "input")
                    .map(|(_, v)| v.into_owned());

                match __raw {
                    Some(ref __s) => match ::serde_json::from_str(__s) {
                        Ok(v) => v,
                        Err(e) => return __rpc_error_response(400,
                            &format!("Failed to deserialize input: {}", e)),
                    },
                    None => match ::serde_json::from_value(::serde_json::Value::Null) {
                        Ok(v) => v,
                        Err(e) => return __rpc_error_response(400,
                            &format!("Missing required input parameter: {}", e)),
                    },
                }
            };
        },
        HandlerKind::Mutation => quote! {
            let __input: #input_type = {
                use ::http_body_util::BodyExt as _;
                let __collected = __req.into_body().collect().await
                    .map_err(|e| ::vercel_runtime::Error::from(
                        format!("Failed to read request body: {}", e)
                    ))?;
                let __bytes = __collected.to_bytes();

                if __bytes.is_empty() {
                    match ::serde_json::from_value(::serde_json::Value::Null) {
                        Ok(v) => v,
                        Err(e) => return __rpc_error_response(400,
                            &format!("Missing required request body: {}", e)),
                    }
                } else {
                    match ::serde_json::from_slice(&__bytes) {
                        Ok(v) => v,
                        Err(e) => return __rpc_error_response(400,
                            &format!("Failed to deserialize request body: {}", e)),
                    }
                }
            };
        },
    };

    // Build the result handling block depending on whether the fn returns Result<T, E>
    let result_handling = if returns_result {
        quote! {
            match __raw_result {
                Ok(__val) => {
                    let __data = ::serde_json::to_value(&__val)
                        .map_err(|e| ::vercel_runtime::Error::from(
                            format!("Failed to serialize response: {}", e)
                        ))?;
                    __rpc_ok_response(__data)
                }
                Err(__err) => {
                    __rpc_error_response(400, &format!("{}", __err))
                }
            }
        }
    } else {
        quote! {
            let __data = ::serde_json::to_value(&__raw_result)
                .map_err(|e| ::vercel_runtime::Error::from(
                    format!("Failed to serialize response: {}", e)
                ))?;
            __rpc_ok_response(__data)
        }
    };

    let expanded = quote! {
        #[tokio::main]
        async fn main() -> Result<(), ::vercel_runtime::Error> {
            ::vercel_runtime::run(::vercel_runtime::service_fn(__rpc_handler)).await
        }

        // Shared CORS headers applied to every response.
        fn __rpc_cors_headers() -> [(&'static str, &'static str); 4] {
            [
                ("Access-Control-Allow-Origin", "*"),
                ("Access-Control-Allow-Methods", "GET, POST, OPTIONS"),
                ("Access-Control-Allow-Headers", "Content-Type"),
                ("Access-Control-Max-Age", "86400"),
            ]
        }

        // Builds a successful JSON response with CORS headers.
        fn __rpc_ok_response(
            data: ::serde_json::Value,
        ) -> Result<::vercel_runtime::Response<::serde_json::Value>, ::vercel_runtime::Error> {
            let mut builder = ::vercel_runtime::Response::builder()
                .status(200)
                .header("Content-Type", "application/json");

            for (k, v) in __rpc_cors_headers() {
                builder = builder.header(k, v);
            }

            Ok(builder.body(::serde_json::json!({
                "result": {
                    "type": "response",
                    "data": data
                }
            }))?)
        }

        // Builds a JSON error response with CORS headers.
        fn __rpc_error_response(
            status: u16,
            message: &str,
        ) -> Result<::vercel_runtime::Response<::serde_json::Value>, ::vercel_runtime::Error> {
            let mut builder = ::vercel_runtime::Response::builder()
                .status(status)
                .header("Content-Type", "application/json");

            for (k, v) in __rpc_cors_headers() {
                builder = builder.header(k, v);
            }

            Ok(builder.body(::serde_json::json!({
                "error": {
                    "type": "error",
                    "message": message
                }
            }))?)
        }

        async fn __rpc_handler(
            __req: ::vercel_runtime::Request,
        ) -> Result<::vercel_runtime::Response<::serde_json::Value>, ::vercel_runtime::Error> {
            // Handle CORS preflight
            if __req.method() == "OPTIONS" {
                let mut builder = ::vercel_runtime::Response::builder()
                    .status(204);
                for (k, v) in __rpc_cors_headers() {
                    builder = builder.header(k, v);
                }
                return Ok(builder.body(::serde_json::Value::Null)?);
            }

            // Validate HTTP method
            if __req.method() != #expected_method {
                return __rpc_error_response(
                    405,
                    &format!(
                        "Method {} not allowed, expected {}",
                        __req.method(),
                        #expected_method,
                    ),
                );
            }

            #parse_input

            async fn #fn_name(#input_pat: #input_type) -> #return_type
            #fn_block

            let __raw_result = #fn_name(__input).await;

            #result_handling
        }
    };

    Ok(expanded)
}

/// Returns `true` if the type syntactically ends with `Result`
/// (e.g. `Result<T, E>`, `std::result::Result<T, E>`).
///
/// **Limitation:** this is a purely syntactic check. Type aliases (e.g.
/// `type MyResult<T> = Result<T, MyError>`) will not be detected, and custom
/// types named `Result` will be falsely identified.
fn is_result_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        return segment.ident == "Result";
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_fn(code: &str) -> ItemFn {
        syn::parse_str(code).unwrap()
    }

    fn parse_type(code: &str) -> Type {
        syn::parse_str(code).unwrap()
    }

    // --- is_result_type ---

    #[test]
    fn result_type_detected() {
        assert!(is_result_type(&parse_type("Result<String, Error>")));
    }

    #[test]
    fn plain_type_not_result() {
        assert!(!is_result_type(&parse_type("String")));
    }

    #[test]
    fn vec_type_not_result() {
        assert!(!is_result_type(&parse_type("Vec<u8>")));
    }

    #[test]
    fn unit_type_not_result() {
        assert!(!is_result_type(&parse_type("()")));
    }

    // --- generate_handler: query ---

    #[test]
    fn query_no_input() {
        let func = parse_fn("async fn version() -> String { \"1.0\".into() }");
        let tokens = build_handler(func, HandlerKind::Query).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("\"GET\""));
        assert!(code.contains("__rpc_handler"));
        assert!(code.contains("__rpc_ok_response"));
    }

    #[test]
    fn query_with_input() {
        let func = parse_fn("async fn hello(name: String) -> String { name }");
        let tokens = build_handler(func, HandlerKind::Query).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("\"GET\""));
        assert!(code.contains("input"));
    }

    #[test]
    fn query_returns_result() {
        let func =
            parse_fn("async fn fetch(id: u32) -> Result<String, String> { Ok(\"ok\".into()) }");
        let tokens = build_handler(func, HandlerKind::Query).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("__rpc_error_response (400"));
        assert!(code.contains("Ok (__val)"));
        assert!(code.contains("Err (__err)"));
    }

    #[test]
    fn query_no_return_type() {
        let func = parse_fn("async fn ping() {}");
        let tokens = build_handler(func, HandlerKind::Query).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("__rpc_ok_response"));
    }

    // --- generate_handler: mutation ---

    #[test]
    fn mutation_with_input() {
        let func = parse_fn("async fn create(input: Data) -> Data { input }");
        let tokens = build_handler(func, HandlerKind::Mutation).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("\"POST\""));
        assert!(code.contains("into_body"));
    }

    #[test]
    fn mutation_no_input() {
        let func = parse_fn("async fn reset() -> u32 { 0 }");
        let tokens = build_handler(func, HandlerKind::Mutation).unwrap();
        let code = tokens.to_string();
        assert!(code.contains("\"POST\""));
    }

    // --- generate_handler: errors ---

    #[test]
    fn rejects_multiple_params() {
        let func = parse_fn("async fn bad(a: String, b: u32) -> String { a }");
        let err = build_handler(func, HandlerKind::Query).unwrap_err();
        assert!(err.to_string().contains("at most one input parameter"));
    }

    #[test]
    fn rejects_non_async_function() {
        let func: ItemFn = syn::parse_str("fn sync_handler() -> String { \"hi\".into() }").unwrap();
        let err = build_handler(func, HandlerKind::Query).unwrap_err();
        assert!(err.to_string().contains("must be async"));
    }

    #[test]
    fn self_receiver_ignored() {
        let func: ItemFn =
            syn::parse_str("async fn method(self, name: String) -> String { name }").unwrap();
        let tokens = build_handler(func, HandlerKind::Query).unwrap();
        let code = tokens.to_string();
        // `self` is filtered out, only `name: String` remains as input
        assert!(code.contains("input"));
    }

    // --- generate_handler: shared structure ---

    #[test]
    fn generates_cors_headers() {
        let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
        let code = build_handler(func, HandlerKind::Query).unwrap().to_string();
        assert!(code.contains("Access-Control-Allow-Origin"));
        assert!(code.contains("Access-Control-Allow-Methods"));
        assert!(code.contains("Access-Control-Max-Age"));
    }

    #[test]
    fn generates_options_handler() {
        let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
        let code = build_handler(func, HandlerKind::Query).unwrap().to_string();
        assert!(code.contains("\"OPTIONS\""));
        assert!(code.contains("204"));
    }

    #[test]
    fn generates_method_not_allowed() {
        let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
        let code = build_handler(func, HandlerKind::Query).unwrap().to_string();
        assert!(code.contains("405"));
        assert!(code.contains("not allowed"));
    }
}
