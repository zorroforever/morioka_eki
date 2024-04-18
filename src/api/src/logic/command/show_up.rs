use std::io;
use gettext::Catalog;
use serde_json::json;
use crate::util;
use crate::util::local_storage_util;

pub(crate) async fn handle(
    catalog: &Catalog
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = local_storage_util::get_global_union_api_url_with_token();
    let acc_id = local_storage_util::get_global_account_id();
    let ch_id = local_storage_util::get_global_character_id();
    let p_data = json!(
        {
            "api_key":"api_show_around",
            "data": {
                    "aid": acc_id,
                    "cid": ch_id,
                    "direction":"up"
            }
        }
    );
    if let Ok(_v) = util::http_util::post(&url, p_data).await {
        println!("{}",_v);
    } else {
        println!("{}", catalog.gettext("token error."));
    }

    Ok(())
}