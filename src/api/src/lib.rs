mod util;
mod service;

use std::io::{self, Write};
use std::path::Path;
use std::process;


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

#[tokio::main]
pub async fn main() -> io::Result<()> {
    load_env();
    let catalog = load_translation();
    println!("{}", catalog.gettext("Welcome to luna world  program!"));

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim() {
            "login" => {
                service::login::handle(&catalog).await.expect("login error!");
            }
            "ck" => {
                service::check_token::handle(&catalog).await.expect("login error!");
            }
            "exit" => {
                println!("{}", catalog.gettext("Exiting program."));
                process::exit(0);
            }
            _ => println!("{}", catalog.gettext("Unknown command, please try again.")),
        }
    }
}

