#![warn(missing_docs)]
//! Procedural macros that turn async Rust functions into
//! [Vercel](https://vercel.com) serverless lambda handlers.
//!
//! Part of the [`vercel-rpc`](https://github.com/misha-mad/vercel-rpc) project
//! — end-to-end typesafe RPC between Rust lambdas on Vercel and any TypeScript frontend.
//!
//! # Quick Start
//!
//! Use the [`vercel-rpc`](https://crates.io/crates/vercel-rpc) facade crate
//! which re-exports these macros together with all runtime dependencies:
//!
//! ```toml
//! [dependencies]
//! vercel-rpc = "0.1"
//! serde      = { version = "1", features = ["derive"] }
//! ```
//!
//! Then annotate an async function:
//!
//! ```rust,ignore
//! use vercel_rpc::rpc_query;
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
//! - A `main()` entry point with a single-threaded tokio runtime that calls `vercel_runtime::run`.
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
/// - A `main()` entry point with a single-threaded tokio runtime that calls `vercel_runtime::run`.
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
/// # Caching
///
/// Use the `cache` attribute to generate `Cache-Control` headers on successful
/// responses. On Vercel this enables edge caching without infrastructure changes.
///
/// ```rust,ignore
/// // CDN caches for 1 hour, browser always revalidates
/// #[rpc_query(cache = "1h")]
/// async fn get_settings() -> Settings { /* ... */ }
/// // → Cache-Control: public, max-age=0, s-maxage=3600
///
/// // CDN caches 5 min, serves stale up to 1 hour while revalidating
/// #[rpc_query(cache = "5m", stale = "1h")]
/// async fn get_feed() -> Vec<Post> { /* ... */ }
/// // → Cache-Control: public, max-age=0, s-maxage=300, stale-while-revalidate=3600
///
/// // Browser-only cache, no CDN
/// #[rpc_query(cache = "private, 10m")]
/// async fn get_profile() -> Profile { /* ... */ }
/// // → Cache-Control: private, max-age=600
/// ```
///
/// Duration shorthand: `30s`, `5m`, `1h`, `1d`. Error responses never receive
/// cache headers. Mutations (`#[rpc_mutation]`) do not support caching.
///
/// # Initialization
///
/// Use the `init` attribute to run an async function once at cold start.
/// The init function can optionally return shared state injected as `&T`:
///
/// ```rust,ignore
/// // Side-effects only (logger, env loading)
/// #[rpc_query(init = "setup")]
/// async fn get_data() -> Data { /* ... */ }
///
/// // With state injection
/// #[rpc_query(init = "setup")]
/// async fn get_user(id: u32, state: &AppState) -> User {
///     state.pool.query("...").await
/// }
///
/// // Combined with cache
/// #[rpc_query(init = "setup", cache = "1h")]
/// async fn get_user(id: u32, state: &AppState) -> User { /* ... */ }
/// ```
///
/// The macro distinguishes state (`&T`) from input (`T`) by reference syntax.
/// A `&T` parameter requires `init`; `&mut T` is rejected.
///
/// # Timeout
///
/// Use the `timeout` attribute to enforce a per-procedure server-side timeout.
/// If the handler does not complete within the specified duration, the request
/// returns a `504` error response with `"Handler timed out"`.
///
/// ```rust,ignore
/// #[rpc_query(timeout = "30s")]
/// async fn slow_query() -> String { /* ... */ }
///
/// #[rpc_query(timeout = "5m", cache = "1h")]
/// async fn expensive(id: u32) -> Report { /* ... */ }
/// ```
///
/// Duration shorthand: `30s`, `5m`, `1h`, `1d`. The timeout is also forwarded
/// to the generated TypeScript client as a per-procedure default.
///
/// # Limitations
///
/// - `Result` and `Headers` are detected by **name only** (last path segment).
///   Type aliases like `type MyResult<T> = Result<T, MyError>` will not be
///   recognized, and custom types named `Result` or `Headers` will be falsely
///   matched. Use the canonical names directly.
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
    let attrs = match parse_handler_attrs(attr) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };
    let input_fn = parse_macro_input!(item as ItemFn);
    build_handler(input_fn, HandlerKind::Query, attrs)
        .map(Into::into)
        .unwrap_or_else(|e| e.to_compile_error().into())
}

/// Generates a Vercel-compatible lambda handler from an async **mutation** function.
///
/// Works like [`rpc_query`] but creates a **POST** endpoint.
/// Input is read from the **JSON request body** instead of query parameters.
///
/// The macro generates:
/// - A `main()` entry point with a single-threaded tokio runtime that calls `vercel_runtime::run`.
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
/// # Initialization
///
/// Mutations support the `init` and `timeout` attributes (but not `cache`):
///
/// ```rust,ignore
/// #[rpc_mutation(init = "setup")]
/// async fn create_order(input: OrderInput, state: &AppState) -> Order {
///     state.pool.query("...").await
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
    let attrs = match parse_handler_attrs(attr) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };
    if attrs.cache_config.is_some() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "rpc_mutation does not support cache/stale attributes",
        )
        .to_compile_error()
        .into();
    }
    let input_fn = parse_macro_input!(item as ItemFn);
    build_handler(input_fn, HandlerKind::Mutation, attrs)
        .map(Into::into)
        .unwrap_or_else(|e| e.to_compile_error().into())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HandlerKind {
    Query,
    Mutation,
}

/// Holds the computed `Cache-Control` header value for a query handler.
#[derive(Debug)]
struct CacheConfig {
    cache_control: String,
}

/// Parsed attributes from `#[rpc_query(...)]` or `#[rpc_mutation(...)]`.
#[derive(Debug)]
struct HandlerAttrs {
    cache_config: Option<CacheConfig>,
    init_fn: Option<String>,
    timeout_secs: Option<u64>,
}

/// Parses optional `init` / `cache` / `stale` key-value pairs from handler attributes.
///
/// Returns `HandlerAttrs` with all fields `None` when the attribute is empty
/// (backward compatible bare `#[rpc_query]`).
fn parse_handler_attrs(attr: TokenStream) -> Result<HandlerAttrs, syn::Error> {
    parse_handler_attrs_inner(attr.into())
}

/// Inner implementation that accepts `proc_macro2::TokenStream` for testability.
fn parse_handler_attrs_inner(attr: proc_macro2::TokenStream) -> Result<HandlerAttrs, syn::Error> {
    if attr.is_empty() {
        return Ok(HandlerAttrs {
            cache_config: None,
            init_fn: None,
            timeout_secs: None,
        });
    }

    let parsed = syn::parse::Parser::parse2(
        syn::punctuated::Punctuated::<syn::MetaNameValue, syn::token::Comma>::parse_terminated,
        attr,
    )?;

    let mut cache_value = None;
    let mut stale_value = None;
    let mut init_value = None;
    let mut timeout_value = None;

    for nv in &parsed {
        let key = nv
            .path
            .get_ident()
            .ok_or_else(|| syn::Error::new_spanned(&nv.path, "expected a simple identifier"))?;

        let value = match &nv.value {
            syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                syn::Lit::Str(s) => s.value(),
                _ => {
                    return Err(syn::Error::new_spanned(
                        &nv.value,
                        "expected a string literal",
                    ));
                }
            },
            _ => {
                return Err(syn::Error::new_spanned(
                    &nv.value,
                    "expected a string literal",
                ));
            }
        };

        if key == "cache" {
            if cache_value.is_some() {
                return Err(syn::Error::new_spanned(key, "duplicate `cache` attribute"));
            }
            cache_value = Some(value);
        } else if key == "stale" {
            if stale_value.is_some() {
                return Err(syn::Error::new_spanned(key, "duplicate `stale` attribute"));
            }
            stale_value = Some(value);
        } else if key == "init" {
            if init_value.is_some() {
                return Err(syn::Error::new_spanned(key, "duplicate `init` attribute"));
            }
            if value.is_empty() {
                return Err(syn::Error::new_spanned(
                    &nv.value,
                    "init function path cannot be empty",
                ));
            }
            init_value = Some(value);
        } else if key == "timeout" {
            if timeout_value.is_some() {
                return Err(syn::Error::new_spanned(
                    key,
                    "duplicate `timeout` attribute",
                ));
            }
            if value.is_empty() {
                return Err(syn::Error::new_spanned(
                    &nv.value,
                    "timeout duration cannot be empty",
                ));
            }
            timeout_value = Some(value);
        } else {
            return Err(syn::Error::new_spanned(
                key,
                format!("unknown attribute `{key}`"),
            ));
        }
    }

    let cache_config = if cache_value.is_some() || stale_value.is_some() {
        let cache_value = cache_value.ok_or_else(|| {
            syn::Error::new(
                proc_macro2::Span::call_site(),
                "missing required `cache` attribute",
            )
        })?;

        let cache_control = build_cache_control(&cache_value, stale_value.as_deref())
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

        Some(CacheConfig { cache_control })
    } else {
        None
    };

    let timeout_secs = timeout_value
        .map(|v| parse_duration(&v))
        .transpose()
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;

    Ok(HandlerAttrs {
        cache_config,
        init_fn: init_value,
        timeout_secs,
    })
}

/// Parses a human-readable duration shorthand into seconds.
///
/// Supported suffixes: `s` (seconds), `m` (minutes), `h` (hours), `d` (days).
/// Zero durations are rejected.
fn parse_duration(s: &str) -> Result<u64, String> {
    if s.is_empty() {
        return Err("duration cannot be empty".into());
    }

    let (num_str, multiplier) = if let Some(n) = s.strip_suffix('s') {
        (n, 1)
    } else if let Some(n) = s.strip_suffix('m') {
        (n, 60)
    } else if let Some(n) = s.strip_suffix('h') {
        (n, 3600)
    } else if let Some(n) = s.strip_suffix('d') {
        (n, 86400)
    } else {
        return Err(format!(
            "invalid duration suffix in `{s}`, expected s/m/h/d"
        ));
    };

    let num: u64 = num_str
        .parse()
        .map_err(|_| format!("invalid number in duration `{s}`"))?;

    if num == 0 {
        return Err(format!("duration cannot be zero: `{s}`"));
    }

    Ok(num * multiplier)
}

/// Builds the `Cache-Control` header value from parsed `cache` and optional `stale` values.
///
/// - `"1h"` → `"public, max-age=0, s-maxage=3600"`
/// - `"private, 10m"` → `"private, max-age=600"`
/// - `"5m"` + stale `"1h"` → `"public, max-age=0, s-maxage=300, stale-while-revalidate=3600"`
fn build_cache_control(cache_value: &str, stale_value: Option<&str>) -> Result<String, String> {
    let (is_private, duration_str) = if let Some(rest) = cache_value.strip_prefix("private,") {
        (true, rest.trim())
    } else {
        (false, cache_value.trim())
    };

    let seconds = parse_duration(duration_str)?;
    let stale_seconds = stale_value.map(parse_duration).transpose()?;

    if is_private {
        let mut header = format!("private, max-age={seconds}");
        if let Some(stale) = stale_seconds {
            header.push_str(&format!(", stale-while-revalidate={stale}"));
        }
        Ok(header)
    } else {
        let mut header = format!("public, max-age=0, s-maxage={seconds}");
        if let Some(stale) = stale_seconds {
            header.push_str(&format!(", stale-while-revalidate={stale}"));
        }
        Ok(header)
    }
}

/// Transforms a user-defined async function into a complete Vercel lambda handler.
///
/// Generates `main()`, CORS helpers, input parsing, and response serialization.
/// The `kind` parameter determines whether the handler accepts GET (query) or POST (mutation).
#[expect(
    clippy::needless_pass_by_value,
    reason = "ItemFn is owned from parse_macro_input"
)]
fn build_handler(
    func: ItemFn,
    kind: HandlerKind,
    attrs: HandlerAttrs,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let HandlerAttrs {
        cache_config,
        init_fn,
        timeout_secs,
    } = attrs;

    if func.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            func.sig.fn_token,
            "RPC handlers must be async functions",
        ));
    }

    let fn_name = &func.sig.ident;
    let fn_block = &func.block;
    let fn_output = &func.sig.output;

    // Separate typed parameters into input, headers, and state params.
    let typed_params: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat) => Some(pat),
            _ => None,
        })
        .collect();

    let mut input_param = None;
    let mut headers_param = None;
    let mut state_param = None;

    for param in &typed_params {
        if is_headers_type(&param.ty) {
            if headers_param.is_some() {
                return Err(syn::Error::new_spanned(
                    &func.sig.inputs,
                    "RPC handlers accept at most one Headers parameter",
                ));
            }
            headers_param = Some(*param);
        } else if let Type::Reference(r) = &*param.ty {
            if r.mutability.is_some() {
                return Err(syn::Error::new_spanned(
                    &param.ty,
                    "state parameter must be a shared reference (&T), not &mut T",
                ));
            }
            if state_param.is_some() {
                return Err(syn::Error::new_spanned(
                    &func.sig.inputs,
                    "RPC handlers accept at most one state parameter",
                ));
            }
            state_param = Some(*param);
        } else if input_param.is_some() {
            return Err(syn::Error::new_spanned(
                &func.sig.inputs,
                "RPC handlers accept at most one input parameter",
            ));
        } else {
            input_param = Some(*param);
        }
    }

    if state_param.is_some() && init_fn.is_none() {
        return Err(syn::Error::new_spanned(
            &func.sig.inputs,
            "state parameter requires init = \"...\" attribute",
        ));
    }

    let input_type = if let Some(param) = input_param {
        let ty = &param.ty;
        quote! { #ty }
    } else {
        quote! { () }
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
                let __url = match ::vercel_rpc::__private::url::Url::parse(
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
                    Some(ref __s) => match ::vercel_rpc::__private::serde_json::from_str(__s) {
                        Ok(v) => v,
                        Err(e) => return __rpc_error_response(400,
                            &format!("Failed to deserialize input: {}", e)),
                    },
                    None => match ::vercel_rpc::__private::serde_json::from_value(::vercel_rpc::__private::serde_json::Value::Null) {
                        Ok(v) => v,
                        Err(e) => return __rpc_error_response(400,
                            &format!("Missing required input parameter: {}", e)),
                    },
                }
            };
        },
        HandlerKind::Mutation => quote! {
            let __input: #input_type = {
                use ::vercel_rpc::__private::http_body_util::BodyExt as _;
                let __collected = __req.into_body().collect().await
                    .map_err(|e| ::vercel_rpc::__private::vercel_runtime::Error::from(
                        format!("Failed to read request body: {}", e)
                    ))?;
                let __bytes = __collected.to_bytes();

                if __bytes.is_empty() {
                    match ::vercel_rpc::__private::serde_json::from_value(::vercel_rpc::__private::serde_json::Value::Null) {
                        Ok(v) => v,
                        Err(e) => return __rpc_error_response(400,
                            &format!("Missing required request body: {}", e)),
                    }
                } else {
                    match ::vercel_rpc::__private::serde_json::from_slice(&__bytes) {
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
                    let __data = ::vercel_rpc::__private::serde_json::to_value(&__val)
                        .map_err(|e| ::vercel_rpc::__private::vercel_runtime::Error::from(
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
            let __data = ::vercel_rpc::__private::serde_json::to_value(&__raw_result)
                .map_err(|e| ::vercel_rpc::__private::vercel_runtime::Error::from(
                    format!("Failed to serialize response: {}", e)
                ))?;
            __rpc_ok_response(__data)
        }
    };

    // Generate headers extraction and function parameters based on Headers presence.
    let extract_headers = if headers_param.is_some() {
        quote! { let __headers = __req.headers().clone(); }
    } else {
        quote! {}
    };

    // Generate state extraction when state param is present.
    let extract_state = if state_param.is_some() {
        quote! { let __state = __RPC_STATE.get().expect("BUG: init not called"); }
    } else {
        quote! {}
    };

    // Build the inner function parameters and call arguments preserving original order.
    let inner_fn_params: Vec<_> = typed_params
        .iter()
        .map(|param| {
            let pat = &param.pat;
            let ty = &param.ty;
            quote! { #pat: #ty }
        })
        .collect();

    let call_args: Vec<_> = typed_params
        .iter()
        .map(|param| {
            if is_headers_type(&param.ty) {
                quote! { __headers }
            } else if is_ref_type(&param.ty) {
                quote! { __state }
            } else {
                quote! { __input }
            }
        })
        .collect();

    let cache_header = match &cache_config {
        Some(config) => {
            let value = &config.cache_control;
            quote! { .header("Cache-Control", #value) }
        }
        None => quote! {},
    };

    // Generate OnceLock static when init returns state.
    let state_static = if let Some(sp) = state_param {
        let Type::Reference(r) = &*sp.ty else {
            unreachable!("state_param is always a reference");
        };
        let inner_ty = &r.elem;
        quote! {
            static __RPC_STATE: std::sync::OnceLock<#inner_ty> = std::sync::OnceLock::new();
        }
    } else {
        quote! {}
    };

    // Generate init call inside block_on.
    let init_call = if let Some(ref path) = init_fn {
        let path_ident: proc_macro2::TokenStream = path.parse().map_err(|_| {
            syn::Error::new_spanned(&func.sig, format!("invalid init function path: `{path}`"))
        })?;
        if state_param.is_some() {
            quote! {
                __RPC_STATE.set(#path_ident().await).expect("BUG: OnceLock already set");
            }
        } else {
            quote! {
                #path_ident().await;
            }
        }
    } else {
        quote! {}
    };

    let invoke_user_fn = if let Some(secs) = timeout_secs {
        quote! {
            match ::vercel_rpc::__private::tokio::time::timeout(
                ::std::time::Duration::from_secs(#secs),
                #fn_name(#(#call_args),*),
            ).await {
                Ok(result) => result,
                Err(_) => return __rpc_error_response(504, "Handler timed out"),
            }
        }
    } else {
        quote! { #fn_name(#(#call_args),*).await }
    };

    let expanded = quote! {
        #state_static

        fn main() -> Result<(), ::vercel_rpc::__private::vercel_runtime::Error> {
            ::vercel_rpc::__private::tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| ::vercel_rpc::__private::vercel_runtime::Error::from(
                    format!("Failed to build tokio runtime: {}", e)
                ))?
                .block_on(async {
                    #init_call
                    ::vercel_rpc::__private::vercel_runtime::run(
                        ::vercel_rpc::__private::vercel_runtime::service_fn(__rpc_handler),
                    ).await
                })
        }

        // Shared CORS headers applied to every response.
        fn __rpc_cors_headers() -> [(&'static str, &'static str); 4] {
            [
                ("Access-Control-Allow-Origin", "*"),
                ("Access-Control-Allow-Methods", "GET, POST, OPTIONS"),
                ("Access-Control-Allow-Headers", "Content-Type, Authorization"),
                ("Access-Control-Max-Age", "86400"),
            ]
        }

        // Builds a successful JSON response with CORS headers.
        fn __rpc_ok_response(
            data: ::vercel_rpc::__private::serde_json::Value,
        ) -> Result<::vercel_rpc::__private::vercel_runtime::Response<::vercel_rpc::__private::serde_json::Value>, ::vercel_rpc::__private::vercel_runtime::Error> {
            let mut builder = ::vercel_rpc::__private::vercel_runtime::Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                #cache_header;

            for (k, v) in __rpc_cors_headers() {
                builder = builder.header(k, v);
            }

            Ok(builder.body(::vercel_rpc::__private::serde_json::json!({
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
        ) -> Result<::vercel_rpc::__private::vercel_runtime::Response<::vercel_rpc::__private::serde_json::Value>, ::vercel_rpc::__private::vercel_runtime::Error> {
            let mut builder = ::vercel_rpc::__private::vercel_runtime::Response::builder()
                .status(status)
                .header("Content-Type", "application/json");

            for (k, v) in __rpc_cors_headers() {
                builder = builder.header(k, v);
            }

            Ok(builder.body(::vercel_rpc::__private::serde_json::json!({
                "error": {
                    "type": "error",
                    "message": message
                }
            }))?)
        }

        async fn __rpc_handler(
            __req: ::vercel_rpc::__private::vercel_runtime::Request,
        ) -> Result<::vercel_rpc::__private::vercel_runtime::Response<::vercel_rpc::__private::serde_json::Value>, ::vercel_rpc::__private::vercel_runtime::Error> {
            // Handle CORS preflight
            if __req.method() == "OPTIONS" {
                let mut builder = ::vercel_rpc::__private::vercel_runtime::Response::builder()
                    .status(204);
                for (k, v) in __rpc_cors_headers() {
                    builder = builder.header(k, v);
                }
                return Ok(builder.body(::vercel_rpc::__private::serde_json::Value::Null)?);
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

            #extract_headers

            #extract_state

            #parse_input

            async fn #fn_name(#(#inner_fn_params),*) -> #return_type
            #fn_block

            let __raw_result = #invoke_user_fn;

            #result_handling
        }
    };

    Ok(expanded)
}

/// Returns `true` if the type syntactically ends with `Headers`
/// (e.g. `Headers`, `vercel_rpc::Headers`).
///
/// **Limitation:** this is a purely syntactic check, similar to
/// [`is_result_type`]. Type aliases will not be detected, and custom
/// types named `Headers` will be falsely identified.
fn is_headers_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        return segment.ident == "Headers";
    }
    false
}

/// Returns `true` if the type is a shared (immutable) reference `&T`.
fn is_ref_type(ty: &Type) -> bool {
    matches!(ty, Type::Reference(r) if r.mutability.is_none())
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
mod tests;
