use std::fs;

use tempfile::TempDir;

use metaxy_cli::commands::{bytecount, cmd_generate, cmd_scan, write_file};
use metaxy_cli::config;

// --- bytecount ---

#[test]
fn bytecount_small() {
    assert_eq!(bytecount("hello"), "5 bytes");
}

#[test]
fn bytecount_empty() {
    assert_eq!(bytecount(""), "0 bytes");
}

#[test]
fn bytecount_kilobytes() {
    let s = "x".repeat(2048);
    assert_eq!(bytecount(&s), "2.0 KB");
}

#[test]
fn bytecount_boundary() {
    let s = "x".repeat(1023);
    assert_eq!(bytecount(&s), "1023 bytes");
}

// --- write_file ---

#[test]
fn write_file_creates_parent_dirs() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("a/b/c/output.ts");
    write_file(&path, "content").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "content");
}

#[test]
fn write_file_overwrites_existing() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("out.ts");
    write_file(&path, "first").unwrap();
    write_file(&path, "second").unwrap();
    assert_eq!(fs::read_to_string(&path).unwrap(), "second");
}

// --- cmd_scan ---

#[test]
fn cmd_scan_empty_dir_errors() {
    let tmp = TempDir::new().unwrap();
    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: tmp.path().to_path_buf(),
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        ..config::RpcConfig::default()
    };
    let err = cmd_scan(&cfg).unwrap_err();
    assert!(err.to_string().contains("No .rs files found"));
}

#[test]
fn cmd_scan_with_procedure() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("hello.rs"),
        r#"
use serde::Serialize;

#[derive(Serialize)]
struct Greeting {
    message: String,
}

#[rpc_query]
async fn hello(name: String) -> Greeting {
    Greeting { message: format!("Hello, {}!", name) }
}
"#,
    )
    .unwrap();

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: tmp.path().to_path_buf(),
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        ..config::RpcConfig::default()
    };
    cmd_scan(&cfg).unwrap();
}

#[test]
fn cmd_scan_with_enum() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("status.rs"),
        r#"
use serde::Serialize;

#[derive(Serialize)]
enum Status {
    Active,
    Inactive,
}

#[rpc_query]
async fn get_status() -> Status {
    Status::Active
}
"#,
    )
    .unwrap();

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: tmp.path().to_path_buf(),
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        ..config::RpcConfig::default()
    };
    cmd_scan(&cfg).unwrap();
}

// --- cmd_generate ---

#[test]
fn cmd_generate_produces_files() {
    let tmp = TempDir::new().unwrap();
    let api_dir = tmp.path().join("api");
    fs::create_dir(&api_dir).unwrap();
    fs::write(
        api_dir.join("ping.rs"),
        r#"
#[rpc_query]
async fn ping() -> String {
    "pong".to_string()
}
"#,
    )
    .unwrap();

    let types_path = tmp.path().join("out/rpc-types.ts");
    let client_path = tmp.path().join("out/rpc-client.ts");

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: api_dir,
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        output: config::OutputConfig {
            types: types_path.clone(),
            client: client_path.clone(),
            svelte: None,
            vue: None,
            ..config::OutputConfig::default()
        },
        ..config::RpcConfig::default()
    };
    cmd_generate(&cfg).unwrap();

    let types = fs::read_to_string(&types_path).unwrap();
    assert!(types.contains("ping"));

    let client = fs::read_to_string(&client_path).unwrap();
    assert!(client.contains("ping"));
}

#[test]
fn cmd_generate_empty_dir_errors() {
    let tmp = TempDir::new().unwrap();
    let types_path = tmp.path().join("rpc-types.ts");
    let client_path = tmp.path().join("rpc-client.ts");

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: tmp.path().to_path_buf(),
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        output: config::OutputConfig {
            types: types_path,
            client: client_path,
            svelte: None,
            vue: None,
            ..config::OutputConfig::default()
        },
        ..config::RpcConfig::default()
    };
    let err = cmd_generate(&cfg).unwrap_err();
    assert!(err.to_string().contains("No .rs files found"));
}

#[test]
fn cmd_generate_writes_svelte_file() {
    let tmp = TempDir::new().unwrap();
    let api_dir = tmp.path().join("api");
    fs::create_dir(&api_dir).unwrap();
    fs::write(
        api_dir.join("ping.rs"),
        r#"
#[rpc_query]
async fn ping() -> String {
    "pong".to_string()
}
"#,
    )
    .unwrap();

    let types_path = tmp.path().join("out/rpc-types.ts");
    let client_path = tmp.path().join("out/rpc-client.ts");
    let svelte_path = tmp.path().join("out/rpc.svelte.ts");

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: api_dir,
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        output: config::OutputConfig {
            types: types_path,
            client: client_path,
            svelte: Some(svelte_path.clone()),
            ..config::OutputConfig::default()
        },
        ..config::RpcConfig::default()
    };
    cmd_generate(&cfg).unwrap();

    let svelte = fs::read_to_string(&svelte_path).unwrap();
    assert!(svelte.contains("createQuery"));
    assert!(svelte.contains("$state"));
    assert!(svelte.contains("$effect"));
}

#[test]
fn cmd_generate_skips_svelte_when_not_configured() {
    let tmp = TempDir::new().unwrap();
    let api_dir = tmp.path().join("api");
    fs::create_dir(&api_dir).unwrap();
    fs::write(
        api_dir.join("ping.rs"),
        r#"
#[rpc_query]
async fn ping() -> String {
    "pong".to_string()
}
"#,
    )
    .unwrap();

    let types_path = tmp.path().join("out/rpc-types.ts");
    let client_path = tmp.path().join("out/rpc-client.ts");
    let svelte_path = tmp.path().join("out/rpc.svelte.ts");

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: api_dir,
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        output: config::OutputConfig {
            types: types_path,
            client: client_path,
            svelte: None, // Not configured
            ..config::OutputConfig::default()
        },
        ..config::RpcConfig::default()
    };
    cmd_generate(&cfg).unwrap();

    assert!(!svelte_path.exists());
}

#[test]
fn cmd_generate_writes_react_file() {
    let tmp = TempDir::new().unwrap();
    let api_dir = tmp.path().join("api");
    fs::create_dir(&api_dir).unwrap();
    fs::write(
        api_dir.join("ping.rs"),
        r#"
#[rpc_query]
async fn ping() -> String {
    "pong".to_string()
}
"#,
    )
    .unwrap();

    let types_path = tmp.path().join("out/rpc-types.ts");
    let client_path = tmp.path().join("out/rpc-client.ts");
    let react_path = tmp.path().join("out/rpc.react.ts");

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: api_dir,
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        output: config::OutputConfig {
            types: types_path,
            client: client_path,
            react: Some(react_path.clone()),
            ..config::OutputConfig::default()
        },
        ..config::RpcConfig::default()
    };
    cmd_generate(&cfg).unwrap();

    let react = fs::read_to_string(&react_path).unwrap();
    assert!(react.contains("useQuery"));
    assert!(react.contains("useState"));
    assert!(react.contains("useEffect"));
}

#[test]
fn cmd_generate_skips_react_when_not_configured() {
    let tmp = TempDir::new().unwrap();
    let api_dir = tmp.path().join("api");
    fs::create_dir(&api_dir).unwrap();
    fs::write(
        api_dir.join("ping.rs"),
        r#"
#[rpc_query]
async fn ping() -> String {
    "pong".to_string()
}
"#,
    )
    .unwrap();

    let types_path = tmp.path().join("out/rpc-types.ts");
    let client_path = tmp.path().join("out/rpc-client.ts");
    let react_path = tmp.path().join("out/rpc.react.ts");

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: api_dir,
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        output: config::OutputConfig {
            types: types_path,
            client: client_path,
            react: None, // Not configured
            ..config::OutputConfig::default()
        },
        ..config::RpcConfig::default()
    };
    cmd_generate(&cfg).unwrap();

    assert!(!react_path.exists());
}

#[test]
fn cmd_generate_writes_vue_file() {
    let tmp = TempDir::new().unwrap();
    let api_dir = tmp.path().join("api");
    fs::create_dir(&api_dir).unwrap();
    fs::write(
        api_dir.join("ping.rs"),
        r#"
#[rpc_query]
async fn ping() -> String {
    "pong".to_string()
}
"#,
    )
    .unwrap();

    let types_path = tmp.path().join("out/rpc-types.ts");
    let client_path = tmp.path().join("out/rpc-client.ts");
    let vue_path = tmp.path().join("out/rpc.vue.ts");

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: api_dir,
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        output: config::OutputConfig {
            types: types_path,
            client: client_path,
            vue: Some(vue_path.clone()),
            ..config::OutputConfig::default()
        },
        ..config::RpcConfig::default()
    };
    cmd_generate(&cfg).unwrap();

    let vue = fs::read_to_string(&vue_path).unwrap();
    assert!(vue.contains("useQuery"));
    assert!(vue.contains("watch("));
    assert!(vue.contains("onScopeDispose"));
}

#[test]
fn cmd_generate_skips_vue_when_not_configured() {
    let tmp = TempDir::new().unwrap();
    let api_dir = tmp.path().join("api");
    fs::create_dir(&api_dir).unwrap();
    fs::write(
        api_dir.join("ping.rs"),
        r#"
#[rpc_query]
async fn ping() -> String {
    "pong".to_string()
}
"#,
    )
    .unwrap();

    let types_path = tmp.path().join("out/rpc-types.ts");
    let client_path = tmp.path().join("out/rpc-client.ts");
    let vue_path = tmp.path().join("out/rpc.vue.ts");

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: api_dir,
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        output: config::OutputConfig {
            types: types_path,
            client: client_path,
            vue: None, // Not configured
            ..config::OutputConfig::default()
        },
        ..config::RpcConfig::default()
    };
    cmd_generate(&cfg).unwrap();

    assert!(!vue_path.exists());
}

#[test]
fn cmd_generate_writes_solid_file() {
    let tmp = TempDir::new().unwrap();
    let api_dir = tmp.path().join("api");
    fs::create_dir(&api_dir).unwrap();
    fs::write(
        api_dir.join("ping.rs"),
        r#"
#[rpc_query]
async fn ping() -> String {
    "pong".to_string()
}
"#,
    )
    .unwrap();

    let types_path = tmp.path().join("out/rpc-types.ts");
    let client_path = tmp.path().join("out/rpc-client.ts");
    let solid_path = tmp.path().join("out/rpc.solid.ts");

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: api_dir,
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        output: config::OutputConfig {
            types: types_path,
            client: client_path,
            solid: Some(solid_path.clone()),
            ..config::OutputConfig::default()
        },
        ..config::RpcConfig::default()
    };
    cmd_generate(&cfg).unwrap();

    let solid = fs::read_to_string(&solid_path).unwrap();
    assert!(solid.contains("createQuery"));
    assert!(solid.contains("createSignal"));
    assert!(solid.contains("createEffect"));
}

#[test]
fn cmd_generate_skips_solid_when_not_configured() {
    let tmp = TempDir::new().unwrap();
    let api_dir = tmp.path().join("api");
    fs::create_dir(&api_dir).unwrap();
    fs::write(
        api_dir.join("ping.rs"),
        r#"
#[rpc_query]
async fn ping() -> String {
    "pong".to_string()
}
"#,
    )
    .unwrap();

    let types_path = tmp.path().join("out/rpc-types.ts");
    let client_path = tmp.path().join("out/rpc-client.ts");
    let solid_path = tmp.path().join("out/rpc.solid.ts");

    let cfg = config::RpcConfig {
        input: config::InputConfig {
            dir: api_dir,
            include: vec!["**/*.rs".into()],
            exclude: vec![],
        },
        output: config::OutputConfig {
            types: types_path,
            client: client_path,
            solid: None, // Not configured
            ..config::OutputConfig::default()
        },
        ..config::RpcConfig::default()
    };
    cmd_generate(&cfg).unwrap();

    assert!(!solid_path.exists());
}
