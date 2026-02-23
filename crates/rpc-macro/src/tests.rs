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
    let tokens = build_handler(func, HandlerKind::Query, None).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("\"GET\""));
    assert!(code.contains("__rpc_handler"));
    assert!(code.contains("__rpc_ok_response"));
}

#[test]
fn query_with_input() {
    let func = parse_fn("async fn hello(name: String) -> String { name }");
    let tokens = build_handler(func, HandlerKind::Query, None).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("\"GET\""));
    assert!(code.contains("input"));
}

#[test]
fn query_returns_result() {
    let func = parse_fn("async fn fetch(id: u32) -> Result<String, String> { Ok(\"ok\".into()) }");
    let tokens = build_handler(func, HandlerKind::Query, None).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("__rpc_error_response (400"));
    assert!(code.contains("Ok (__val)"));
    assert!(code.contains("Err (__err)"));
}

#[test]
fn query_no_return_type() {
    let func = parse_fn("async fn ping() {}");
    let tokens = build_handler(func, HandlerKind::Query, None).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("__rpc_ok_response"));
}

// --- generate_handler: mutation ---

#[test]
fn mutation_with_input() {
    let func = parse_fn("async fn create(input: Data) -> Data { input }");
    let tokens = build_handler(func, HandlerKind::Mutation, None).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("\"POST\""));
    assert!(code.contains("into_body"));
}

#[test]
fn mutation_no_input() {
    let func = parse_fn("async fn reset() -> u32 { 0 }");
    let tokens = build_handler(func, HandlerKind::Mutation, None).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("\"POST\""));
}

// --- generate_handler: errors ---

#[test]
fn rejects_multiple_params() {
    let func = parse_fn("async fn bad(a: String, b: u32) -> String { a }");
    let err = build_handler(func, HandlerKind::Query, None).unwrap_err();
    assert!(err.to_string().contains("at most one input parameter"));
}

#[test]
fn rejects_non_async_function() {
    let func: ItemFn = syn::parse_str("fn sync_handler() -> String { \"hi\".into() }").unwrap();
    let err = build_handler(func, HandlerKind::Query, None).unwrap_err();
    assert!(err.to_string().contains("must be async"));
}

#[test]
fn self_receiver_ignored() {
    let func: ItemFn =
        syn::parse_str("async fn method(self, name: String) -> String { name }").unwrap();
    let tokens = build_handler(func, HandlerKind::Query, None).unwrap();
    let code = tokens.to_string();
    // `self` is filtered out, only `name: String` remains as input
    assert!(code.contains("input"));
}

// --- generate_handler: shared structure ---

#[test]
fn generates_cors_headers() {
    let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
    let code = build_handler(func, HandlerKind::Query, None)
        .unwrap()
        .to_string();
    assert!(code.contains("Access-Control-Allow-Origin"));
    assert!(code.contains("Access-Control-Allow-Methods"));
    assert!(code.contains("Access-Control-Max-Age"));
}

#[test]
fn generates_options_handler() {
    let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
    let code = build_handler(func, HandlerKind::Query, None)
        .unwrap()
        .to_string();
    assert!(code.contains("\"OPTIONS\""));
    assert!(code.contains("204"));
}

#[test]
fn generates_current_thread_runtime() {
    let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
    let code = build_handler(func, HandlerKind::Query, None)
        .unwrap()
        .to_string();
    assert!(code.contains("new_current_thread"));
    assert!(!code.contains("tokio :: main"));
}

#[test]
fn generates_method_not_allowed() {
    let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
    let code = build_handler(func, HandlerKind::Query, None)
        .unwrap()
        .to_string();
    assert!(code.contains("405"));
    assert!(code.contains("not allowed"));
}

// --- parse_duration ---

#[test]
fn duration_seconds() {
    assert_eq!(parse_duration("30s").unwrap(), 30);
}

#[test]
fn duration_minutes() {
    assert_eq!(parse_duration("5m").unwrap(), 300);
}

#[test]
fn duration_hours() {
    assert_eq!(parse_duration("1h").unwrap(), 3600);
}

#[test]
fn duration_days() {
    assert_eq!(parse_duration("1d").unwrap(), 86400);
}

#[test]
fn duration_rejects_zero() {
    assert!(parse_duration("0s").unwrap_err().contains("zero"));
}

#[test]
fn duration_rejects_invalid_suffix() {
    assert!(parse_duration("10x").unwrap_err().contains("suffix"));
}

#[test]
fn duration_rejects_empty() {
    assert!(parse_duration("").unwrap_err().contains("empty"));
}

#[test]
fn duration_rejects_no_number() {
    assert!(parse_duration("h").unwrap_err().contains("invalid number"));
}

// --- build_cache_control ---

#[test]
fn cache_control_public_default() {
    let header = build_cache_control("1h", None).unwrap();
    assert_eq!(header, "public, max-age=0, s-maxage=3600");
}

#[test]
fn cache_control_public_with_stale() {
    let header = build_cache_control("5m", Some("1h")).unwrap();
    assert_eq!(
        header,
        "public, max-age=0, s-maxage=300, stale-while-revalidate=3600"
    );
}

#[test]
fn cache_control_private() {
    let header = build_cache_control("private, 10m", None).unwrap();
    assert_eq!(header, "private, max-age=600");
}

#[test]
fn cache_control_private_no_s_maxage() {
    let header = build_cache_control("private, 1h", None).unwrap();
    assert!(!header.contains("s-maxage"));
}

#[test]
fn cache_control_private_with_stale() {
    let header = build_cache_control("private, 5m", Some("1h")).unwrap();
    assert_eq!(header, "private, max-age=300, stale-while-revalidate=3600");
}

// --- generate_handler: caching ---

#[test]
fn query_with_cache_header() {
    let func = parse_fn("async fn get_settings() -> String { String::new() }");
    let config = Some(CacheConfig {
        cache_control: "public, max-age=0, s-maxage=3600".into(),
    });
    let code = build_handler(func, HandlerKind::Query, config)
        .unwrap()
        .to_string();
    assert!(code.contains("Cache-Control"));
    assert!(code.contains("s-maxage=3600"));
}

#[test]
fn query_with_stale_while_revalidate() {
    let func = parse_fn("async fn get_feed() -> String { String::new() }");
    let config = Some(CacheConfig {
        cache_control: "public, max-age=0, s-maxage=300, stale-while-revalidate=3600".into(),
    });
    let code = build_handler(func, HandlerKind::Query, config)
        .unwrap()
        .to_string();
    assert!(code.contains("stale-while-revalidate=3600"));
}

#[test]
fn query_with_private_cache() {
    let func = parse_fn("async fn get_profile() -> String { String::new() }");
    let config = Some(CacheConfig {
        cache_control: "private, max-age=600".into(),
    });
    let code = build_handler(func, HandlerKind::Query, config)
        .unwrap()
        .to_string();
    assert!(code.contains("private, max-age=600"));
    assert!(!code.contains("s-maxage"));
}

#[test]
fn query_without_cache_no_header() {
    let func = parse_fn("async fn plain() -> String { String::new() }");
    let code = build_handler(func, HandlerKind::Query, None)
        .unwrap()
        .to_string();
    assert!(!code.contains("Cache-Control"));
}

#[test]
fn mutation_never_has_cache_header() {
    let func = parse_fn("async fn create(input: String) -> String { input }");
    let code = build_handler(func, HandlerKind::Mutation, None)
        .unwrap()
        .to_string();
    assert!(!code.contains("Cache-Control"));
}

#[test]
fn error_response_never_has_cache_header() {
    let func = parse_fn("async fn risky(id: u32) -> Result<String, String> { Ok(\"ok\".into()) }");
    let config = Some(CacheConfig {
        cache_control: "public, max-age=0, s-maxage=3600".into(),
    });
    let code = build_handler(func, HandlerKind::Query, config)
        .unwrap()
        .to_string();
    // Cache-Control appears in __rpc_ok_response but not __rpc_error_response
    let ok_section = code.split("__rpc_error_response").next().unwrap();
    let err_section = code.split("__rpc_ok_response").last().unwrap();
    assert!(ok_section.contains("Cache-Control"));
    assert!(!err_section.contains("Cache-Control"));
}

// --- parse_cache_attrs ---

#[test]
fn parse_cache_attrs_empty_returns_none() {
    let result = parse_cache_attrs_inner(quote! {}).unwrap();
    assert!(result.is_none());
}

#[test]
fn parse_cache_attrs_valid_cache() {
    let result = parse_cache_attrs_inner(quote! { cache = "1h" }).unwrap();
    let config = result.unwrap();
    assert_eq!(config.cache_control, "public, max-age=0, s-maxage=3600");
}

#[test]
fn parse_cache_attrs_cache_with_stale() {
    let result = parse_cache_attrs_inner(quote! { cache = "5m", stale = "1h" }).unwrap();
    let config = result.unwrap();
    assert_eq!(
        config.cache_control,
        "public, max-age=0, s-maxage=300, stale-while-revalidate=3600"
    );
}

#[test]
fn parse_cache_attrs_rejects_unknown_key() {
    let err = parse_cache_attrs_inner(quote! { cache = "1h", timeout = "30s" }).unwrap_err();
    assert!(err.to_string().contains("unknown attribute"));
}

#[test]
fn parse_cache_attrs_rejects_missing_cache() {
    let err = parse_cache_attrs_inner(quote! { stale = "1h" }).unwrap_err();
    assert!(err.to_string().contains("missing required `cache`"));
}

#[test]
fn parse_cache_attrs_rejects_non_string_literal() {
    let err = parse_cache_attrs_inner(quote! { cache = 3600 }).unwrap_err();
    assert!(err.to_string().contains("expected a string literal"));
}

#[test]
fn parse_cache_attrs_rejects_duplicate_cache() {
    let err = parse_cache_attrs_inner(quote! { cache = "1h", cache = "2h" }).unwrap_err();
    assert!(err.to_string().contains("duplicate `cache`"));
}

#[test]
fn parse_cache_attrs_rejects_duplicate_stale() {
    let err =
        parse_cache_attrs_inner(quote! { cache = "1h", stale = "1h", stale = "2h" }).unwrap_err();
    assert!(err.to_string().contains("duplicate `stale`"));
}
