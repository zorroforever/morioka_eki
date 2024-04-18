use std::io;
use std::io::Write;
use gettext::Catalog;

use crate::util::local_storage_util;

pub(crate) async fn handle(
    catalog: &Catalog
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut input_str = String::new();

    let mut ch_id = String::new();
    print!("{}", catalog.gettext("Please enter character id: "));
    io::stdout().flush()?;
    io::stdin().read_line(&mut input_str)?;

    ch_id = input_str.trim().to_string();
    if let Ok(cid) = ch_id.parse::<i32>() {
        local_storage_util::set_global_character_id(cid);
        // TODO init character on the map.
        // set character position.
        println!("{} is selected.",cid);
    } else {
        println!("{}", catalog.gettext("character id is un valid."));
    }

    Ok(())
}