mod database;

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
    let input = req.query_string_parameters_ref()
        .and_then(|params| params.first("input"))
        .unwrap_or_else(|| "")
        .to_string();

    let results = database::find(input);
    // TODO filter out input from results

    let resp = json!(results);

    Ok(resp)
}
