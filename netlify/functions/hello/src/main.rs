use lambda_http::{run, http::{StatusCode, Response}, service_fn, Body, Error, IntoResponse, Request, RequestExt};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    let func = service_fn(handler);
    run(func).await
}

pub(crate) async fn handler(req: Request) -> Result<impl IntoResponse, Error> {
    let path = req.raw_http_path();

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(Body::from(format!("Hello from '{}'", path)))
        .map_err(Box::new)?;

    Ok(resp)
}
