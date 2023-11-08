mod database;

use http::header::CONTENT_TYPE;
use lambda_http::{run, service_fn, Body, Error, IntoResponse, Request, RequestExt, Response};
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
        .unwrap_or_else(|| "");

    let results = database::find(input);

    let filtered_results = results
        .iter()
        .filter(|x| x.to_uppercase() != input.to_uppercase())
        .collect::<Vec<_>>();

    let json_body = json!(filtered_results);

    let string_body = serde_json::to_string(&json_body)
        .expect("unable to serialize JSON body");

    let body = Body::from(string_body);

    let resp = Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .expect("unable to build http::Response");

    Ok(resp)
}
