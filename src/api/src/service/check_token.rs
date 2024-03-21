use std::{env, io};
use std::future::Future;
use std::io::Write;
use gettext::Catalog;
use serde_json::json;
use crate::util;

pub(crate) async fn handle(
    catalog: &Catalog
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    print!("{}", catalog.gettext("Please enter token: "));
    io::stdout().flush()?;
    let mut token = String::new();
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str)?;
    token = input_str.trim().to_string();
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
    let api_url = env::var("API_UNION").unwrap_or_else(|_| "/service/".to_string());
    let url = format!("{}{}{}", base_url,api_url,token);
    let p_data = json!({"api_key":"api_check_token","data": token});
    if let Ok(_v) = util::http_util::post(&url,p_data).await {
        println!("{}", catalog.gettext("token is valid."));
    } else {
        println!("{}",  catalog.gettext("token error."));
    }
    Ok(())
}