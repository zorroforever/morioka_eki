use std::io::{self, Write};
use std::path::Path;
use std::{env, process};
use crate::util::local_storage_util;

mod util;
mod logic;

fn load_env() {
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

fn init()-> (){
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
    let api_url = env::var("API_UNION").unwrap_or_else(|_| "/logic/".to_string());
    let api_union_url = format!("{}{}",base_url,api_url);
    local_storage_util::set_global_api_url(api_union_url);
    local_storage_util::set_global_base_url(base_url);
}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    load_env();
    let catalog = load_translation();
    println!("{}", catalog.gettext("Welcome to luna world  program!"));
    init();
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "login" => {
                logic::command::login::handle(&catalog).await.expect("login error!");
            }
            "ck" => {
                logic::command::check_token::handle(&catalog).await.expect("check error!");
            }
            "mk_character" => {
                logic::command::create_character::handle(&catalog).await.expect("mk_character error!");
            }
            "exit" => {
                println!("{}", catalog.gettext("Exiting program."));
                process::exit(0);
            }
            _ => println!("{}", catalog.gettext("Unknown command, please try again.")),
        }
    }
}

