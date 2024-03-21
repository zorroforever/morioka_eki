use std::{env, io};
use std::io::Write;
use aes::cipher::KeyInit;
use gettext::Catalog;
use rand::thread_rng;
use serde_json::Value;
use crate::util;
use crate::util::local_storage_util;

pub(crate) async fn handle(
    catalog: &Catalog
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut rng = thread_rng();
    // let random_string: String = rng.sample_iter(&Alphanumeric).take(8).map(char::from).collect();
    print!("{}", catalog.gettext("Please enter username: "));
    io::stdout().flush()?;
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    print!("{}", catalog.gettext("Please enter password: "));
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let mut credentials = format!("{}{}", username.trim(), password.trim());
    // credentials.push_str(&random_string);
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
    let key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set in .env");
    let encrypted_data = util::crypto_util::aes_encrypt(&credentials.into_bytes(), &key.into_bytes()).await;

    // let request_data = format!("{}{}", encrypted_data, &random_string);
    let api_url = env::var("API_FETCH_TOKEN").unwrap_or_else(|_| "/logic/".to_string());
    let url = format!("{}{}{}", base_url,api_url,encrypted_data);

    match util::http_util::get(&url).await {
        Ok(token) => {
            let parsed_json: Result<Value, _> = serde_json::from_str(&token);
            if let Ok(json) = parsed_json {
                // 获取 token 值
                if let Some(token_value) = json.get("token") {
                    // 将值转换为字符串
                    if let Some(token_str) = token_value.as_str() {
                        local_storage_util::set_global_token(token_str.to_string());
                        println!("{}",  catalog.gettext("Login success."));
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{}", catalog.gettext("Login failed."));
        }
    }
    Ok(())
}