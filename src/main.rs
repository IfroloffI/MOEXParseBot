mod utils;
use crate::utils::json_processing::json::*;

mod crates;
use crate::crates::bot::bot::*;
// use crate::crates::

fn main() {
    pretty_env_logger::init();

    let secure_data: serde_json::Value = get_private_data();
    let token: String = get_token(&secure_data);

    let _error = start_bot(token);
    log::info!("Stoping bot...");
}
