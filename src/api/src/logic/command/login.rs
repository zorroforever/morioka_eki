use std::io::Write;
use std::{env, io};

use aes::cipher::KeyInit;
use gettext::Catalog;
use serde_json::Value;

use crate::util;
use crate::util::local_storage_util;

pub(crate) async fn handle(
    catalog: &Catalog,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    let base_url = local_storage_util::get_global_base_url();
    let key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set in .env");
    let encrypted_data =
        util::crypto_util::aes_encrypt(&credentials.into_bytes(), &key.into_bytes()).await;

    // let request_data = format!("{}{}", encrypted_data, &random_string);
    let api_url = env::var("API_FETCH_TOKEN").unwrap_or_else(|_| "/logic/".to_string());
    let url = format!("{}{}{}", base_url, api_url, encrypted_data);

    match util::http_util::get(&url).await {
        Ok(token) => {
            let parsed_json: Result<Value, _> = serde_json::from_str(&token);
            if let Ok(json) = parsed_json {
                if let Some(token_value) = json.get("token") {
                    if let Some(token_str) = token_value.as_str() {
                        local_storage_util::set_global_token(token_str.to_string());
                        println!("{}", catalog.gettext("Login success."));
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
