use std::io;
use std::io::Write;
use gettext::Catalog;
use serde_json::json;
use crate::util;
use crate::util::local_storage_util;

pub(crate) async fn handle(
    catalog: &Catalog
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut input_str = String::new();

    let mut ch_name = String::new();
    print!("{}", catalog.gettext("Please enter character name: "));
    io::stdout().flush()?;
    io::stdin().read_line(&mut input_str)?;
    ch_name = input_str.trim().to_string();

    let mut input_str1 = String::new();
    let mut ch_nickname = String::new();
    print!("{}", catalog.gettext("Please enter character nickname: "));
    io::stdout().flush()?;
    io::stdin().read_line(&mut input_str1)?;
    ch_nickname = input_str1.trim().to_string();

    let url = local_storage_util::get_global_union_api_url_with_token();
    let p_data = json!(
        {
            "api_key":"api_create_character",
            "data": {
                    "ch_name": ch_name,
                    "ch_nickname": ch_nickname
            }
        }
    );
    if let Ok(_v) = util::http_util::post(&url, p_data).await {
        println!("{}", catalog.gettext("token is valid."));
    } else {
        println!("{}", catalog.gettext("token error."));
    }
    Ok(())
}