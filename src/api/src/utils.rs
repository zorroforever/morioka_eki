use aes::cipher::{BlockEncrypt, KeyInit};
use aes::cipher::generic_array::GenericArray;
use reqwest::Client;

pub async fn encrypt_data(data: &str, key: &str) -> String {
    let cipher = aes::Aes256::new(GenericArray::from_slice(key.as_bytes()));
    let mut buffer = [0u8; 16];
    buffer[..data.len()].copy_from_slice(data.as_bytes());
    cipher.encrypt_block(&mut GenericArray::from_mut_slice(&mut buffer));
    buffer.iter().map(|&byte| format!("{:02X}", byte)).collect::<String>()
}

pub async fn http_get(
    url: &str
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    if res.status().is_success() {
        let body = res.text().await?;
        Ok(body)
    } else {
        Err("Request failed.".into())
    }
}


