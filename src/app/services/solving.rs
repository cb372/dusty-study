use gloo_net::http::Request;
use gloo_net::Error;

pub async fn solve(input: String) -> Result<Vec<String>, Error> {
    let url = format!("/.netlify/functions/solve?input={}", input);
    Request::get(&url)
        .send()
        .await?
        .json()
        .await
}

pub async fn warmup() -> Result<Vec<String>, Error> {
    solve("DUSTY".to_string()).await
}
