# Example: camelCase field naming

This example shows how to use `[codegen.naming]` to convert snake_case Rust field names into camelCase TypeScript property names.

## Configuration

```toml
# rpc.config.toml
[codegen.naming]
fields = "camelCase"
```

See [rpc.config.toml](./rpc.config.toml) for the full config.

## Rust source

```rust
// api/status.rs
#[derive(Serialize)]
pub struct ServiceStatus {
    pub uptime_secs: u64,
    pub api_version: String,
    pub is_healthy: bool,
}
```

## Generated TypeScript

With `fields = "camelCase"`, the CLI produces:

```typescript
export interface ServiceStatus {
  uptimeSecs: number;
  apiVersion: string;
  isHealthy: boolean;
}
```

With `fields = "preserve"` (the default), field names stay as-is:

```typescript
export interface ServiceStatus {
  uptime_secs: number;
  api_version: string;
  is_healthy: boolean;
}
```

## Try it

From the repository root:

```bash
cargo run -p vercel-rpc-cli -- generate \
  --config examples/camel-case-fields/rpc.config.toml \
  --dir examples/camel-case-fields/api \
  --output examples/camel-case-fields/output/rpc-types.ts \
  --client-output examples/camel-case-fields/output/rpc-client.ts
```

Compare the output with [expected-output.ts](./expected-output.ts).
