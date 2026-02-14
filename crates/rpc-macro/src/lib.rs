use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, PatType, ReturnType, Type};

/// Generates a Vercel-compatible lambda handler from an RPC query function.
///
/// The macro wraps the annotated function with:
/// - A `main()` entry point compatible with `vercel_runtime`
/// - Automatic JSON deserialization of the `input` query parameter (GET)
/// - Automatic JSON serialization of the return value
/// - CORS headers for cross-origin requests
/// - HTTP method validation (GET only, with OPTIONS preflight)
/// - Standardized error response format for `Result<T, E>` return types
///
/// # Example
///
/// ```rust,ignore
/// use vercel_rpc_macro::rpc_query;
///
/// #[rpc_query]
/// async fn hello(name: String) -> String {
///     format!("Hello, {}!", name)
/// }
/// ```
#[proc_macro_attribute]
pub fn rpc_query(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    generate_handler(input_fn, HandlerKind::Query)
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Generates a Vercel-compatible lambda handler from an RPC mutation function.
///
/// Same as `#[rpc_query]` but reads input from the request body (POST)
/// instead of query parameters (GET).
///
/// # Example
///
/// ```rust,ignore
/// use vercel_rpc_macro::rpc_mutation;
///
/// #[rpc_mutation]
/// async fn create_user(input: CreateUserInput) -> User {
///     // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn rpc_mutation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    generate_handler(input_fn, HandlerKind::Mutation)
        .unwrap_or_else(|e| e.to_compile_error().into())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HandlerKind {
    Query,
    Mutation,
}

fn generate_handler(func: ItemFn, kind: HandlerKind) -> Result<TokenStream, syn::Error> {
    let fn_name = &func.sig.ident;
    let fn_block = &func.block;
    let fn_output = &func.sig.output;

    let params: Vec<&PatType> = func
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat) => Some(pat),
            _ => None,
        })
        .collect();

    // Validate: at most one parameter allowed
    if params.len() > 1 {
        return Err(syn::Error::new_spanned(
            &func.sig.inputs,
            "RPC handlers accept at most one input parameter",
        ));
    }

    let has_input = !params.is_empty();

    let input_type = if has_input {
        let ty = &params[0].ty;
        quote! { #ty }
    } else {
        quote! { () }
    };

    let input_pat = if has_input {
        let pat = &params[0].pat;
        quote! { #pat }
    } else {
        quote! { _ }
    };

    // Determine whether the handler returns Result<T, E> or plain T
    let (return_type, returns_result) = match fn_output {
        ReturnType::Default => (quote! { () }, false),
        ReturnType::Type(_, ty) => {
            if is_result_type(ty) {
                (quote! { #ty }, true)
            } else {
                (quote! { #ty }, false)
            }
        }
    };

    let expected_method = match kind {
        HandlerKind::Query => "GET",
        HandlerKind::Mutation => "POST",
    };

    let parse_input = match kind {
        HandlerKind::Query => quote! {
            let __input: #input_type = {
                let __url = ::url::Url::parse(
                    &format!("http://localhost{}", __req.uri())
                ).map_err(|e| ::vercel_runtime::Error::from(e.to_string()))?;

                let __raw = __url
                    .query_pairs()
                    .find(|(k, _)| k == "input")
                    .map(|(_, v)| v.into_owned());

                match __raw {
                    Some(ref __s) => ::serde_json::from_str(__s)
                        .map_err(|e| ::vercel_runtime::Error::from(
                            format!("Failed to deserialize input: {}", e)
                        ))?,
                    None => ::serde_json::from_value(::serde_json::Value::Null)
                        .map_err(|e| ::vercel_runtime::Error::from(
                            format!("Missing required input parameter: {}", e)
                        ))?,
                }
            };
        },
        HandlerKind::Mutation => quote! {
            let __input: #input_type = {
                let __body = __req.body();
                let __bytes: &[u8] = __body.as_ref();

                if __bytes.is_empty() {
                    ::serde_json::from_value(::serde_json::Value::Null)
                        .map_err(|e| ::vercel_runtime::Error::from(
                            format!("Missing required request body: {}", e)
                        ))?
                } else {
                    ::serde_json::from_slice(__bytes)
                        .map_err(|e| ::vercel_runtime::Error::from(
                            format!("Failed to deserialize request body: {}", e)
                        ))?
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

    Ok(expanded.into())
}

/// Returns `true` if the type is `Result<T, E>`.
fn is_result_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Result";
        }
    }
    false
}
