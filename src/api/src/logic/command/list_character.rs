use std::io;
use gettext::Catalog;
use serde_json::{json, Value};
use crate::util;
use crate::util::local_storage_util;

pub(crate) async fn handle(
    catalog: &Catalog
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = local_storage_util::get_global_union_api_url_with_token();
    let acc_id = local_storage_util::get_global_account_id();
    let p_data = json!(
        {
            "api_key":"api_list_character",
            "data": {
                    "aid": acc_id
            }
        }
    );
    if let Ok(_v) = util::http_util::post(&url, p_data).await {
        let json_array: Value = serde_json::from_str(&_v)?;

        if let Value::Array(array) = json_array {
            for json_object in array {
                print_json_object(&json_object);
                println!("---"); // 分隔每个 JSON 对象
            }
        } else {
            println!("Expected a JSON array");
        }
    } else {
        println!("{}", catalog.gettext("please create one, use command [mk_ch]."));
    }
    Ok(())
}

fn print_json_object(json: &Value) {
    if let Value::Object(map) = json {
        for (key, value) in map {
            println!("{}: {}", key, value);
        }
    }
}