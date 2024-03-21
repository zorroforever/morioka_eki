use reqwest::Client;

pub async fn get(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    handle_response(res).await
}

pub(crate) async fn post(
    url: &str,
    json_data: serde_json::Value,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let response = client.post(url).json(&json_data).send().await?;
    handle_response(response).await
}

async fn handle_response(
    res: reqwest::Response,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    if res.status().is_success() {
        Ok(res.text().await?)
    } else {
        Err("Request failed.".into())
    }
}
