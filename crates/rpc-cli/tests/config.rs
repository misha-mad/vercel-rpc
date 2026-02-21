use std::path::PathBuf;

use tempfile::TempDir;

use vercel_rpc_cli::config::*;

#[test]
fn test_default_matches_current_behavior() {
    let config = RpcConfig::default();
    assert_eq!(config.input.dir, PathBuf::from("api"));
    assert_eq!(config.input.include, vec!["**/*.rs".to_string()]);
    assert!(config.input.exclude.is_empty());
    assert_eq!(config.output.types, PathBuf::from("src/lib/rpc-types.ts"));
    assert_eq!(config.output.client, PathBuf::from("src/lib/rpc-client.ts"));
    assert!(config.output.svelte.is_none());
    assert!(config.output.react.is_none());
    assert!(config.output.vue.is_none());
    assert!(config.output.solid.is_none());
    assert_eq!(config.output.imports.types_path, "./rpc-types");
    assert_eq!(config.output.imports.extension, "");
    assert_eq!(config.output.imports.types_specifier(), "./rpc-types");
    assert!(!config.codegen.preserve_docs);
    assert_eq!(config.codegen.naming.fields, FieldNaming::Preserve);
    assert_eq!(config.watch.debounce_ms, 200);
    assert!(!config.watch.clear_screen);
}

#[test]
fn test_parse_minimal() {
    let toml_str = r#"
[input]
dir = "src/api"
"#;
    let config: RpcConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.input.dir, PathBuf::from("src/api"));
    // Rest should be defaults
    assert_eq!(config.output.types, PathBuf::from("src/lib/rpc-types.ts"));
    assert_eq!(config.output.client, PathBuf::from("src/lib/rpc-client.ts"));
    assert_eq!(config.output.imports.types_path, "./rpc-types");
    assert_eq!(config.watch.debounce_ms, 200);
}

#[test]
fn test_parse_full() {
    let toml_str = r#"
[input]
dir = "lambdas"
include = ["src/**/*.rs"]
exclude = ["src/tests/**"]

[output]
types = "types.ts"
client = "client.ts"

[output.imports]
types_path = "./types"
extension = ".js"

[codegen]
preserve_docs = true

[codegen.naming]
fields = "camelCase"

[watch]
debounce_ms = 500
clear_screen = true
"#;
    let config: RpcConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.input.dir, PathBuf::from("lambdas"));
    assert_eq!(config.input.include, vec!["src/**/*.rs".to_string()]);
    assert_eq!(config.input.exclude, vec!["src/tests/**".to_string()]);
    assert_eq!(config.output.types, PathBuf::from("types.ts"));
    assert_eq!(config.output.client, PathBuf::from("client.ts"));
    assert_eq!(config.output.imports.types_path, "./types");
    assert_eq!(config.output.imports.extension, ".js");
    assert_eq!(config.output.imports.types_specifier(), "./types.js");
    assert!(config.codegen.preserve_docs);
    assert_eq!(config.codegen.naming.fields, FieldNaming::CamelCase);
    assert_eq!(config.watch.debounce_ms, 500);
    assert!(config.watch.clear_screen);
}

#[test]
fn test_parse_include_exclude() {
    let toml_str = r#"
[input]
include = ["handlers/**/*.rs", "api/**/*.rs"]
exclude = ["**/test_*.rs"]
"#;
    let config: RpcConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.input.dir, PathBuf::from("api")); // default
    assert_eq!(
        config.input.include,
        vec!["handlers/**/*.rs".to_string(), "api/**/*.rs".to_string()]
    );
    assert_eq!(config.input.exclude, vec!["**/test_*.rs".to_string()]);
}

#[test]
fn test_parse_codegen_preserve_docs() {
    let toml_str = r#"
[codegen]
preserve_docs = true
"#;
    let config: RpcConfig = toml::from_str(toml_str).unwrap();
    assert!(config.codegen.preserve_docs);
    // Other fields should be defaults
    assert_eq!(config.input.dir, PathBuf::from("api"));
    assert_eq!(config.output.types, PathBuf::from("src/lib/rpc-types.ts"));
}

#[test]
fn test_parse_codegen_naming_fields() {
    let toml_str = r#"
[codegen.naming]
fields = "camelCase"
"#;
    let config: RpcConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.codegen.naming.fields, FieldNaming::CamelCase);
    // Other fields should be defaults
    assert!(!config.codegen.preserve_docs);
    assert_eq!(config.input.dir, PathBuf::from("api"));

    // Test preserve value
    let toml_str2 = r#"
[codegen.naming]
fields = "preserve"
"#;
    let config2: RpcConfig = toml::from_str(toml_str2).unwrap();
    assert_eq!(config2.codegen.naming.fields, FieldNaming::Preserve);

    // Test default (no naming section)
    let toml_str3 = r#"
[codegen]
preserve_docs = true
"#;
    let config3: RpcConfig = toml::from_str(toml_str3).unwrap();
    assert_eq!(config3.codegen.naming.fields, FieldNaming::Preserve);
}

#[test]
fn test_parse_extension() {
    let toml_str = r#"
[output.imports]
extension = ".js"
"#;
    let config: RpcConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.output.imports.types_path, "./rpc-types"); // default
    assert_eq!(config.output.imports.extension, ".js");
    assert_eq!(config.output.imports.types_specifier(), "./rpc-types.js");
}

#[test]
fn test_parse_empty() {
    let config: RpcConfig = toml::from_str("").unwrap();
    assert_eq!(config.input.dir, PathBuf::from("api"));
    assert_eq!(config.output.types, PathBuf::from("src/lib/rpc-types.ts"));
    assert_eq!(config.output.client, PathBuf::from("src/lib/rpc-client.ts"));
    assert_eq!(config.output.imports.types_path, "./rpc-types");
    assert_eq!(config.watch.debounce_ms, 200);
}

#[test]
fn test_parse_invalid_toml() {
    let result: Result<RpcConfig, _> = toml::from_str("not = [valid toml");
    assert!(result.is_err());
}

#[test]
fn test_unknown_fields_ignored() {
    let toml_str = r#"
[input]
dir = "api"
some_future_field = true

[future_section]
key = "value"
"#;
    // serde(default) doesn't deny unknown fields by default, so this should work
    let result: Result<RpcConfig, _> = toml::from_str(toml_str);
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.input.dir, PathBuf::from("api"));
}

#[test]
fn test_discovery_found() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join(CONFIG_FILE_NAME);
    std::fs::write(&config_path, "[input]\ndir = \"api\"\n").unwrap();

    let found = discover(tmp.path());
    assert_eq!(found, Some(config_path));
}

#[test]
fn test_discovery_parent() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join(CONFIG_FILE_NAME);
    std::fs::write(&config_path, "[input]\ndir = \"api\"\n").unwrap();

    let child = tmp.path().join("sub").join("dir");
    std::fs::create_dir_all(&child).unwrap();

    let found = discover(&child);
    assert_eq!(found, Some(config_path));
}

#[test]
fn test_discovery_missing() {
    let tmp = TempDir::new().unwrap();
    let found = discover(tmp.path());
    assert!(found.is_none());
}

#[test]
fn test_cli_overrides() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join(CONFIG_FILE_NAME);
    std::fs::write(
        &config_path,
        r#"
[input]
dir = "lambdas"

[output]
types = "types.ts"
client = "client.ts"
"#,
    )
    .unwrap();

    let config = load(&config_path).unwrap();
    assert_eq!(config.input.dir, PathBuf::from("lambdas"));

    // Now test that CLI overrides work
    let overrides = CliOverrides {
        config: Some(config_path),
        no_config: false,
        dir: Some(PathBuf::from("other")),
        include: vec!["handlers/**/*.rs".into()],
        exclude: vec!["**/test_*.rs".into()],
        output: Some(PathBuf::from("out.ts")),
        client_output: None,
        svelte_output: None,
        react_output: None,
        vue_output: None,
        solid_output: None,
        types_import: Some("./my-types".to_string()),
        extension: Some(".js".to_string()),
        preserve_docs: true,
        fields: Some(FieldNaming::CamelCase),
        debounce_ms: Some(500),
        clear_screen: true,
    };

    let config = resolve(overrides).unwrap();

    assert_eq!(config.input.dir, PathBuf::from("other"));
    assert_eq!(config.input.include, vec!["handlers/**/*.rs".to_string()]);
    assert_eq!(config.input.exclude, vec!["**/test_*.rs".to_string()]);
    assert_eq!(config.output.types, PathBuf::from("out.ts"));
    assert_eq!(config.output.client, PathBuf::from("client.ts")); // not overridden
    assert_eq!(config.output.imports.types_path, "./my-types");
    assert_eq!(config.output.imports.extension, ".js");
    assert!(config.codegen.preserve_docs);
    assert_eq!(config.codegen.naming.fields, FieldNaming::CamelCase);
    assert_eq!(config.watch.debounce_ms, 500);
    assert!(config.watch.clear_screen);
}

#[test]
fn test_resolve_no_config_flag() {
    let overrides = CliOverrides {
        config: None,
        no_config: true,
        dir: Some(PathBuf::from("custom")),
        include: vec![],
        exclude: vec![],
        output: None,
        client_output: None,
        svelte_output: None,
        react_output: None,
        vue_output: None,
        solid_output: None,
        types_import: None,
        extension: None,
        preserve_docs: false,
        fields: None,
        debounce_ms: None,
        clear_screen: false,
    };
    let config = resolve(overrides).unwrap();
    assert_eq!(config.input.dir, PathBuf::from("custom"));
    // Everything else should be defaults
    assert_eq!(config.output.types, PathBuf::from("src/lib/rpc-types.ts"));
}

#[test]
fn test_resolve_client_output_override() {
    let overrides = CliOverrides {
        config: None,
        no_config: true,
        dir: None,
        include: vec![],
        exclude: vec![],
        output: None,
        client_output: Some(PathBuf::from("custom-client.ts")),
        svelte_output: None,
        react_output: None,
        vue_output: None,
        solid_output: None,
        types_import: None,
        extension: None,
        preserve_docs: false,
        fields: None,
        debounce_ms: None,
        clear_screen: false,
    };
    let config = resolve(overrides).unwrap();
    assert_eq!(config.output.client, PathBuf::from("custom-client.ts"));
}

#[test]
fn test_config_svelte_default_none() {
    let config = RpcConfig::default();
    assert!(config.output.svelte.is_none());
}

#[test]
fn test_config_svelte_parsed() {
    let toml_str = r#"
[output]
svelte = "src/lib/rpc.svelte.ts"
"#;
    let config: RpcConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(
        config.output.svelte,
        Some(PathBuf::from("src/lib/rpc.svelte.ts"))
    );
}

#[test]
fn test_cli_svelte_override() {
    let overrides = CliOverrides {
        config: None,
        no_config: true,
        svelte_output: Some(PathBuf::from("custom.svelte.ts")),
        ..CliOverrides::default()
    };
    let config = resolve(overrides).unwrap();
    assert_eq!(
        config.output.svelte,
        Some(PathBuf::from("custom.svelte.ts"))
    );
}

#[test]
fn test_config_react_default_none() {
    let config = RpcConfig::default();
    assert!(config.output.react.is_none());
}

#[test]
fn test_config_react_parsed() {
    let toml_str = r#"
[output]
react = "src/lib/rpc.react.ts"
"#;
    let config: RpcConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(
        config.output.react,
        Some(PathBuf::from("src/lib/rpc.react.ts"))
    );
}

#[test]
fn test_cli_react_override() {
    let overrides = CliOverrides {
        config: None,
        no_config: true,
        react_output: Some(PathBuf::from("custom.react.ts")),
        ..CliOverrides::default()
    };
    let config = resolve(overrides).unwrap();
    assert_eq!(config.output.react, Some(PathBuf::from("custom.react.ts")));
}

#[test]
fn test_config_vue_default_none() {
    let config = RpcConfig::default();
    assert!(config.output.vue.is_none());
}

#[test]
fn test_config_vue_parsed() {
    let toml_str = r#"
[output]
vue = "src/lib/rpc.vue.ts"
"#;
    let config: RpcConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(config.output.vue, Some(PathBuf::from("src/lib/rpc.vue.ts")));
}

#[test]
fn test_cli_vue_override() {
    let overrides = CliOverrides {
        config: None,
        no_config: true,
        vue_output: Some(PathBuf::from("custom.vue.ts")),
        ..CliOverrides::default()
    };
    let config = resolve(overrides).unwrap();
    assert_eq!(config.output.vue, Some(PathBuf::from("custom.vue.ts")));
}

#[test]
fn test_config_solid_default_none() {
    let config = RpcConfig::default();
    assert!(config.output.solid.is_none());
}

#[test]
fn test_config_solid_parsed() {
    let toml_str = r#"
[output]
solid = "src/lib/rpc.solid.ts"
"#;
    let config: RpcConfig = toml::from_str(toml_str).unwrap();
    assert_eq!(
        config.output.solid,
        Some(PathBuf::from("src/lib/rpc.solid.ts"))
    );
}

#[test]
fn test_cli_solid_override() {
    let overrides = CliOverrides {
        config: None,
        no_config: true,
        solid_output: Some(PathBuf::from("custom.solid.ts")),
        ..CliOverrides::default()
    };
    let config = resolve(overrides).unwrap();
    assert_eq!(
        config.output.solid,
        Some(PathBuf::from("custom.solid.ts"))
    );
}
