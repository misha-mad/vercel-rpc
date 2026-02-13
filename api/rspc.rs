use rspc::{Router, Config};
use std::path::PathBuf;
use vercel_runtime::{run, service_fn, Error, Request, Response};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "export" {
        let router = mount();
        router
            .export_ts(PathBuf::from("./src/lib/rspc.ts"))
            .expect("Failed to export typescript bindings");
        println!("Bindings exported to ./src/lib/rspc.ts");
        return Ok(());
    }

    let service = service_fn(handler);
    run(service).await
}

pub fn mount() -> Router<()> {
    Router::<()>::new()
        .config(Config::new().export_ts_bindings(PathBuf::from("./src/lib/rspc.ts")))
        .query("version", |t| t(|_ctx, _: ()| "0.1.0"))
        .query("hello", |t| {
            t(|_ctx, name: String| format!("Hello, {} from rspc!", name))
        })
        .build()
}

async fn handler(_req: Request) -> Result<Response<serde_json::Value>, Error> {
    let _router = mount();
    
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(json!({ "message": "rspc handler active. Use it with rspc-client." }))?)
}
