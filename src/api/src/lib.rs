use std::io::{self, Write};
use std::path::Path;
use reqwest::Client;
use std::env;
use std::process;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
fn load_env() {
    // 加载 .env 文件中的配置
    dotenv::dotenv().ok();
}

fn load_translation() -> gettext::Catalog {
    let mo_file = Path::new("src/api/i18n/messages.mo");
    println!("Trying to load translation from: {:?}", mo_file);
    if let Ok(bytes) = std::fs::read(mo_file) {
        if let Ok(cat) = gettext::Catalog::parse(&*bytes) {
            return cat;
        }
    }else {
        println!("Failed to load translation: {:?}", std::io::Error::last_os_error());
    }
    gettext::Catalog::empty()
}

fn encrypt_data(data: &str, key: &str) -> String {
    let cipher = aes::Aes256::new(GenericArray::from_slice(key.as_bytes()));
    let mut buffer = [0u8; 16];
    buffer[..data.len()].copy_from_slice(data.as_bytes());
    cipher.encrypt_block(&mut GenericArray::from_mut_slice(&mut buffer));
    buffer.iter().map(|&byte| format!("{:02X}", byte)).collect::<String>()
}


#[tokio::main]
pub async fn main() -> io::Result<()> {
    load_env();
    let catalog = load_translation();
    println!("{}", catalog.gettext("Welcome to luna world  program!"));
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
    let key = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set in .env");
    let mut rng = thread_rng();
    let random_string: String = rng.sample_iter(&Alphanumeric).take(8).map(char::from).collect();
    let mut rl = rustyline::Editor::<()>::new();
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            "login" => {
                print!("{}", catalog.gettext("Please enter username: "));
                io::stdout().flush()?;
                let mut username = String::new();
                io::stdin().read_line(&mut username)?;
                print!("{}", catalog.gettext("Please enter password: "));
                io::stdout().flush()?;
                let mut password = String::new();
                io::stdin().read_line(&mut password)?;
                let mut credentials = format!("{}{}", username.trim(), password.trim());
                credentials.push_str(&random_string);

                // 使用 AES 加密
                let encrypted_data = encrypt_data(&credentials, &key);

                // 构造请求字符串
                let request_data = format!("{}{}", encrypted_data, &random_string);
                let api_url = env::var("API_FETCH_TOKEN").unwrap_or_else(|_| "/api/".to_string());
                let url = format!("{}{}{}", base_url,api_url,request_data);

                match http_get(&url).await {
                    Ok(token) => {
                        println!("{}", token);
                    }
                    Err(e) => {
                        eprintln!("{}", catalog.gettext("Request failed."));
                    }
                }
            }
            "exit" => {
                println!("{}", catalog.gettext("Exiting program."));
                process::exit(0);
            }
            _ => println!("{}", catalog.gettext("Unknown command, please try again.")),
        }
    }
}

async fn http_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    if res.status().is_success() {
        let body = res.text().await?;
        Ok(body)
    } else {
        Err("Request failed.".into())
    }
}
