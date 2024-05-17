use crate::util;
use crate::util::local_storage_util;
use gettext::Catalog;
use serde_json::json;
use std::io;
use std::io::Write;

pub(crate) async fn handle(
    catalog: &Catalog,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = local_storage_util::get_global_union_api_url_with_token();
    let acc_id = local_storage_util::get_global_account_id();
    let ch_id = local_storage_util::get_global_character_id();
    print!("{}", catalog.gettext("Please enter number: "));
    print!("{}{}", "1.", catalog.gettext("block on up."));
    print!("{}{}", "2.", catalog.gettext("block on down."));
    print!("{}{}", "3.", catalog.gettext("block on left."));
    print!("{}{}", "4.", catalog.gettext("block on right."));
    io::stdout().flush()?;
    let mut sel_num = String::new();
    io::stdin().read_line(&mut sel_num)?;
    let mut res = "".to_string();
    match sel_num.trim().parse::<u8>() {
        Ok(1) => {
            res =  show_block(catalog, url, acc_id, ch_id, "up").await.unwrap();
        }
        Ok(2) => {
            res = show_block(catalog, url, acc_id, ch_id, "down").await.unwrap();
        }
        Ok(3) => {
            res = show_block(catalog, url, acc_id, ch_id, "left").await.unwrap();
        }
        Ok(4) => {
            res = show_block(catalog, url, acc_id, ch_id, "right").await.unwrap();
        }
        _ => {
            println!("{}", catalog.gettext("Invalid input."));
        }
    }
    println!("{} {}", catalog.gettext("Result:"), res);
    Ok(())
}

async fn show_block(catalog: &Catalog, url: String, acc_id: i32, ch_id: i32, direction: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let p_data = json!(
        {
            "api_key":"api_show_around",
            "data": {
                    "aid": acc_id,
                    "cid": ch_id,
                    "direction": direction
            }
        }
    );
    if let Ok(_v) = util::http_util::post(&url, p_data).await {
        println!("{}", _v);
         Ok(_v)
    } else {
        println!("{}", catalog.gettext("token error."));
        Ok("".to_string())
    }
}