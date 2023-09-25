use lambda_http::{run, service_fn, Error, IntoResponse, Request, RequestExt};
use log::LevelFilter;
use serde_json::json;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let func = service_fn(handler);
    run(func).await
}

pub(crate) async fn handler(req: Request) -> Result<impl IntoResponse, Error> {
    let _input = req.query_string_parameters_ref()
        .and_then(|params| params.first("input"))
        .unwrap_or_else(|| "")
        .to_string();

    // TODO convert input to key
    // TODO lookup in hashmap
    // TODO filter out input from results

    let resp = json!(vec!["CAT", "ACT"]);

    Ok(resp)
}
