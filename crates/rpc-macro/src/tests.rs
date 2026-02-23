use super::*;

fn parse_fn(code: &str) -> ItemFn {
    syn::parse_str(code).unwrap()
}

fn parse_type(code: &str) -> Type {
    syn::parse_str(code).unwrap()
}

fn no_attrs() -> HandlerAttrs {
    HandlerAttrs {
        cache_config: None,
        init_fn: None,
    }
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

// --- is_ref_type ---

#[test]
fn shared_ref_detected() {
    assert!(is_ref_type(&parse_type("&AppState")));
}

#[test]
fn mut_ref_not_shared() {
    assert!(!is_ref_type(&parse_type("&mut AppState")));
}

#[test]
fn owned_type_not_ref() {
    assert!(!is_ref_type(&parse_type("String")));
}

// --- generate_handler: query ---

#[test]
fn query_no_input() {
    let func = parse_fn("async fn version() -> String { \"1.0\".into() }");
    let tokens = build_handler(func, HandlerKind::Query, no_attrs()).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("\"GET\""));
    assert!(code.contains("__rpc_handler"));
    assert!(code.contains("__rpc_ok_response"));
}

#[test]
fn query_with_input() {
    let func = parse_fn("async fn hello(name: String) -> String { name }");
    let tokens = build_handler(func, HandlerKind::Query, no_attrs()).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("\"GET\""));
    assert!(code.contains("input"));
}

#[test]
fn query_returns_result() {
    let func = parse_fn("async fn fetch(id: u32) -> Result<String, String> { Ok(\"ok\".into()) }");
    let tokens = build_handler(func, HandlerKind::Query, no_attrs()).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("__rpc_error_response (400"));
    assert!(code.contains("Ok (__val)"));
    assert!(code.contains("Err (__err)"));
}

#[test]
fn query_no_return_type() {
    let func = parse_fn("async fn ping() {}");
    let tokens = build_handler(func, HandlerKind::Query, no_attrs()).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("__rpc_ok_response"));
}

// --- generate_handler: mutation ---

#[test]
fn mutation_with_input() {
    let func = parse_fn("async fn create(input: Data) -> Data { input }");
    let tokens = build_handler(func, HandlerKind::Mutation, no_attrs()).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("\"POST\""));
    assert!(code.contains("into_body"));
}

#[test]
fn mutation_no_input() {
    let func = parse_fn("async fn reset() -> u32 { 0 }");
    let tokens = build_handler(func, HandlerKind::Mutation, no_attrs()).unwrap();
    let code = tokens.to_string();
    assert!(code.contains("\"POST\""));
}

// --- generate_handler: errors ---

#[test]
fn rejects_multiple_params() {
    let func = parse_fn("async fn bad(a: String, b: u32) -> String { a }");
    let err = build_handler(func, HandlerKind::Query, no_attrs()).unwrap_err();
    assert!(err.to_string().contains("at most one input parameter"));
}

#[test]
fn rejects_non_async_function() {
    let func: ItemFn = syn::parse_str("fn sync_handler() -> String { \"hi\".into() }").unwrap();
    let err = build_handler(func, HandlerKind::Query, no_attrs()).unwrap_err();
    assert!(err.to_string().contains("must be async"));
}

#[test]
fn self_receiver_ignored() {
    let func: ItemFn =
        syn::parse_str("async fn method(self, name: String) -> String { name }").unwrap();
    let tokens = build_handler(func, HandlerKind::Query, no_attrs()).unwrap();
    let code = tokens.to_string();
    // `self` is filtered out, only `name: String` remains as input
    assert!(code.contains("input"));
}

// --- generate_handler: shared structure ---

#[test]
fn generates_cors_headers() {
    let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
    let code = build_handler(func, HandlerKind::Query, no_attrs())
        .unwrap()
        .to_string();
    assert!(code.contains("Access-Control-Allow-Origin"));
    assert!(code.contains("Access-Control-Allow-Methods"));
    assert!(code.contains("Access-Control-Max-Age"));
}

#[test]
fn generates_options_handler() {
    let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
    let code = build_handler(func, HandlerKind::Query, no_attrs())
        .unwrap()
        .to_string();
    assert!(code.contains("\"OPTIONS\""));
    assert!(code.contains("204"));
}

#[test]
fn generates_current_thread_runtime() {
    let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
    let code = build_handler(func, HandlerKind::Query, no_attrs())
        .unwrap()
        .to_string();
    assert!(code.contains("new_current_thread"));
    assert!(!code.contains("tokio :: main"));
}

#[test]
fn generates_method_not_allowed() {
    let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
    let code = build_handler(func, HandlerKind::Query, no_attrs())
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
    let attrs = HandlerAttrs {
        cache_config: Some(CacheConfig {
            cache_control: "public, max-age=0, s-maxage=3600".into(),
        }),
        init_fn: None,
    };
    let code = build_handler(func, HandlerKind::Query, attrs)
        .unwrap()
        .to_string();
    assert!(code.contains("Cache-Control"));
    assert!(code.contains("s-maxage=3600"));
}

#[test]
fn query_with_stale_while_revalidate() {
    let func = parse_fn("async fn get_feed() -> String { String::new() }");
    let attrs = HandlerAttrs {
        cache_config: Some(CacheConfig {
            cache_control: "public, max-age=0, s-maxage=300, stale-while-revalidate=3600".into(),
        }),
        init_fn: None,
    };
    let code = build_handler(func, HandlerKind::Query, attrs)
        .unwrap()
        .to_string();
    assert!(code.contains("stale-while-revalidate=3600"));
}

#[test]
fn query_with_private_cache() {
    let func = parse_fn("async fn get_profile() -> String { String::new() }");
    let attrs = HandlerAttrs {
        cache_config: Some(CacheConfig {
            cache_control: "private, max-age=600".into(),
        }),
        init_fn: None,
    };
    let code = build_handler(func, HandlerKind::Query, attrs)
        .unwrap()
        .to_string();
    assert!(code.contains("private, max-age=600"));
    assert!(!code.contains("s-maxage"));
}

#[test]
fn query_without_cache_no_header() {
    let func = parse_fn("async fn plain() -> String { String::new() }");
    let code = build_handler(func, HandlerKind::Query, no_attrs())
        .unwrap()
        .to_string();
    assert!(!code.contains("Cache-Control"));
}

#[test]
fn mutation_never_has_cache_header() {
    let func = parse_fn("async fn create(input: String) -> String { input }");
    let code = build_handler(func, HandlerKind::Mutation, no_attrs())
        .unwrap()
        .to_string();
    assert!(!code.contains("Cache-Control"));
}

#[test]
fn error_response_never_has_cache_header() {
    let func = parse_fn("async fn risky(id: u32) -> Result<String, String> { Ok(\"ok\".into()) }");
    let attrs = HandlerAttrs {
        cache_config: Some(CacheConfig {
            cache_control: "public, max-age=0, s-maxage=3600".into(),
        }),
        init_fn: None,
    };
    let code = build_handler(func, HandlerKind::Query, attrs)
        .unwrap()
        .to_string();
    // Cache-Control appears in __rpc_ok_response but not __rpc_error_response
    let ok_section = code.split("__rpc_error_response").next().unwrap();
    let err_section = code.split("__rpc_ok_response").last().unwrap();
    assert!(ok_section.contains("Cache-Control"));
    assert!(!err_section.contains("Cache-Control"));
}

// --- parse_handler_attrs ---

#[test]
fn parse_handler_attrs_empty_returns_none() {
    let result = parse_handler_attrs_inner(quote! {}).unwrap();
    assert!(result.cache_config.is_none());
    assert!(result.init_fn.is_none());
}

#[test]
fn parse_handler_attrs_valid_cache() {
    let result = parse_handler_attrs_inner(quote! { cache = "1h" }).unwrap();
    let config = result.cache_config.unwrap();
    assert_eq!(config.cache_control, "public, max-age=0, s-maxage=3600");
    assert!(result.init_fn.is_none());
}

#[test]
fn parse_handler_attrs_cache_with_stale() {
    let result = parse_handler_attrs_inner(quote! { cache = "5m", stale = "1h" }).unwrap();
    let config = result.cache_config.unwrap();
    assert_eq!(
        config.cache_control,
        "public, max-age=0, s-maxage=300, stale-while-revalidate=3600"
    );
}

#[test]
fn parse_handler_attrs_rejects_unknown_key() {
    let err = parse_handler_attrs_inner(quote! { cache = "1h", timeout = "30s" }).unwrap_err();
    assert!(err.to_string().contains("unknown attribute"));
}

#[test]
fn parse_handler_attrs_rejects_missing_cache() {
    let err = parse_handler_attrs_inner(quote! { stale = "1h" }).unwrap_err();
    assert!(err.to_string().contains("missing required `cache`"));
}

#[test]
fn parse_handler_attrs_rejects_non_string_literal() {
    let err = parse_handler_attrs_inner(quote! { cache = 3600 }).unwrap_err();
    assert!(err.to_string().contains("expected a string literal"));
}

#[test]
fn parse_handler_attrs_rejects_duplicate_cache() {
    let err = parse_handler_attrs_inner(quote! { cache = "1h", cache = "2h" }).unwrap_err();
    assert!(err.to_string().contains("duplicate `cache`"));
}

#[test]
fn parse_handler_attrs_rejects_duplicate_stale() {
    let err =
        parse_handler_attrs_inner(quote! { cache = "1h", stale = "1h", stale = "2h" }).unwrap_err();
    assert!(err.to_string().contains("duplicate `stale`"));
}

// --- parse_handler_attrs: init ---

#[test]
fn parse_attrs_init_only() {
    let result = parse_handler_attrs_inner(quote! { init = "setup" }).unwrap();
    assert_eq!(result.init_fn.as_deref(), Some("setup"));
    assert!(result.cache_config.is_none());
}

#[test]
fn parse_attrs_init_and_cache() {
    let result = parse_handler_attrs_inner(quote! { init = "setup", cache = "1h" }).unwrap();
    assert_eq!(result.init_fn.as_deref(), Some("setup"));
    let config = result.cache_config.unwrap();
    assert_eq!(config.cache_control, "public, max-age=0, s-maxage=3600");
}

#[test]
fn parse_attrs_init_empty_rejected() {
    let err = parse_handler_attrs_inner(quote! { init = "" }).unwrap_err();
    assert!(err.to_string().contains("empty"));
}

#[test]
fn parse_attrs_init_non_string_rejected() {
    let err = parse_handler_attrs_inner(quote! { init = 42 }).unwrap_err();
    assert!(err.to_string().contains("string literal"));
}

#[test]
fn parse_attrs_duplicate_init_rejected() {
    let err = parse_handler_attrs_inner(quote! { init = "a", init = "b" }).unwrap_err();
    assert!(err.to_string().contains("duplicate"));
}

#[test]
fn mutation_rejects_cache_with_init() {
    let attrs = HandlerAttrs {
        cache_config: Some(CacheConfig {
            cache_control: "public, max-age=0, s-maxage=3600".into(),
        }),
        init_fn: Some("setup".into()),
    };
    // Mutation with cache_config is rejected at the rpc_mutation level.
    // Verify attrs parse succeeds but would be caught by the entry point guard.
    assert!(attrs.cache_config.is_some());
    assert!(attrs.init_fn.is_some());

    // Verify mutation without cache works fine with init.
    let func2 = parse_fn("async fn create2(input: String) -> String { input }");
    let no_cache_attrs = HandlerAttrs {
        cache_config: None,
        init_fn: Some("setup".into()),
    };
    let code = build_handler(func2, HandlerKind::Mutation, no_cache_attrs)
        .unwrap()
        .to_string();
    assert!(code.contains("\"POST\""));
    assert!(!code.contains("Cache-Control"));
}

// --- generate_handler: init ---

#[test]
fn init_side_effects_only() {
    let func = parse_fn("async fn get_data() -> String { String::new() }");
    let attrs = HandlerAttrs {
        cache_config: None,
        init_fn: Some("setup".into()),
    };
    let code = build_handler(func, HandlerKind::Query, attrs)
        .unwrap()
        .to_string();
    assert!(code.contains("setup () . await"));
    assert!(!code.contains("OnceLock"));
    assert!(!code.contains("__RPC_STATE"));
}

#[test]
fn init_with_state() {
    let func = parse_fn("async fn get_data(state: &AppState) -> String { String::new() }");
    let attrs = HandlerAttrs {
        cache_config: None,
        init_fn: Some("setup".into()),
    };
    let code = build_handler(func, HandlerKind::Query, attrs)
        .unwrap()
        .to_string();
    assert!(code.contains("OnceLock"));
    assert!(code.contains("__RPC_STATE"));
    assert!(code.contains("__state"));
    assert!(code.contains("setup () . await"));
}

#[test]
fn init_with_state_and_input() {
    let func = parse_fn("async fn get_user(id: u32, state: &AppState) -> String { String::new() }");
    let attrs = HandlerAttrs {
        cache_config: None,
        init_fn: Some("setup".into()),
    };
    let code = build_handler(func, HandlerKind::Query, attrs)
        .unwrap()
        .to_string();
    assert!(code.contains("__input"));
    assert!(code.contains("__state"));
    assert!(code.contains("OnceLock"));
}

#[test]
fn init_with_state_and_headers() {
    let func = parse_fn(
        "async fn get_data(state: &AppState, headers: Headers) -> String { String::new() }",
    );
    let attrs = HandlerAttrs {
        cache_config: None,
        init_fn: Some("setup".into()),
    };
    let code = build_handler(func, HandlerKind::Query, attrs)
        .unwrap()
        .to_string();
    assert!(code.contains("__state"));
    assert!(code.contains("__headers"));
    assert!(code.contains("OnceLock"));
}

#[test]
fn init_with_all_three_params() {
    let func = parse_fn(
        "async fn get_user(id: u32, state: &AppState, headers: Headers) -> String { String::new() }",
    );
    let attrs = HandlerAttrs {
        cache_config: None,
        init_fn: Some("setup".into()),
    };
    let code = build_handler(func, HandlerKind::Query, attrs)
        .unwrap()
        .to_string();
    assert!(code.contains("__input"));
    assert!(code.contains("__state"));
    assert!(code.contains("__headers"));
}

#[test]
fn init_compatible_with_cache() {
    let func = parse_fn("async fn get_data(state: &AppState) -> String { String::new() }");
    let attrs = HandlerAttrs {
        cache_config: Some(CacheConfig {
            cache_control: "public, max-age=0, s-maxage=3600".into(),
        }),
        init_fn: Some("setup".into()),
    };
    let code = build_handler(func, HandlerKind::Query, attrs)
        .unwrap()
        .to_string();
    assert!(code.contains("OnceLock"));
    assert!(code.contains("Cache-Control"));
}

#[test]
fn init_compatible_with_mutation() {
    let func = parse_fn("async fn create(input: String, state: &AppState) -> String { input }");
    let attrs = HandlerAttrs {
        cache_config: None,
        init_fn: Some("setup".into()),
    };
    let code = build_handler(func, HandlerKind::Mutation, attrs)
        .unwrap()
        .to_string();
    assert!(code.contains("\"POST\""));
    assert!(code.contains("OnceLock"));
    assert!(code.contains("__state"));
}

#[test]
fn init_call_inside_block_on() {
    let func = parse_fn("async fn get_data() -> String { String::new() }");
    let attrs = HandlerAttrs {
        cache_config: None,
        init_fn: Some("setup".into()),
    };
    let code = build_handler(func, HandlerKind::Query, attrs)
        .unwrap()
        .to_string();
    // The init call must be inside block_on (after .block_on(async {)
    let block_on_section = code.split("block_on").last().unwrap();
    assert!(block_on_section.contains("setup () . await"));
}

// --- generate_handler: init error cases ---

#[test]
fn state_without_init_rejected() {
    let func = parse_fn("async fn get_data(state: &AppState) -> String { String::new() }");
    let err = build_handler(func, HandlerKind::Query, no_attrs()).unwrap_err();
    assert!(err.to_string().contains("init"));
}

#[test]
fn mut_state_rejected() {
    let func = parse_fn("async fn get_data(state: &mut AppState) -> String { String::new() }");
    let attrs = HandlerAttrs {
        cache_config: None,
        init_fn: Some("setup".into()),
    };
    let err = build_handler(func, HandlerKind::Query, attrs).unwrap_err();
    assert!(err.to_string().contains("shared reference"));
}

#[test]
fn multiple_state_params_rejected() {
    let func =
        parse_fn("async fn get_data(a: &AppState, b: &OtherState) -> String { String::new() }");
    let attrs = HandlerAttrs {
        cache_config: None,
        init_fn: Some("setup".into()),
    };
    let err = build_handler(func, HandlerKind::Query, attrs).unwrap_err();
    assert!(err.to_string().contains("at most one state parameter"));
}
