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
    let func = parse_fn("async fn fetch(id: u32) -> Result<String, String> { Ok(\"ok\".into()) }");
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
fn generates_current_thread_runtime() {
    let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
    let code = build_handler(func, HandlerKind::Query).unwrap().to_string();
    assert!(code.contains("new_current_thread"));
    assert!(!code.contains("tokio :: main"));
}

#[test]
fn generates_method_not_allowed() {
    let func = parse_fn("async fn ping() -> String { \"pong\".into() }");
    let code = build_handler(func, HandlerKind::Query).unwrap().to_string();
    assert!(code.contains("405"));
    assert!(code.contains("not allowed"));
}
