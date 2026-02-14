use rspc::{Router, Config, ExecKind};
use std::path::PathBuf;
use vercel_runtime::{run, service_fn, Error, Request, Response};
use serde_json::{json, Value};
use url::Url;

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

async fn handler(req: Request) -> Result<Response<Value>, Error> {
    let router = mount();
    let url = Url::parse(&format!("http://localhost{}", req.uri()))?;
    
    let path = url.path();
    let procedure_name = path.strip_prefix("/api/rspc/").unwrap_or("");

    if procedure_name.is_empty() {
        return Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(json!({ "message": "rspc handler active. Use it with rspc-client." }))?);
    }

    let params = url.query_pairs()
        .find(|(key, _)| key == "input")
        .map(|(_, value)| serde_json::from_str::<Value>(&value))
        .transpose()?
        .unwrap_or(Value::Null);

    let result = router.exec((), ExecKind::Query, procedure_name.to_string(), Some(params)).await;

    match result {
        Ok(v) => Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(json!({
                "result": {
                    "type": "response",
                    "data": v
                }
            }))?),
        Err(e) => Ok(Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(json!({
                "result": {
                    "type": "error",
                    "data": {
                        "code": 500,
                        "message": e.to_string()
                    }
                }
            }))?),
    }
}
