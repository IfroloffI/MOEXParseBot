mod utils;
use crate::utils::json_processing::json::*;

mod crates;
use crate::crates::bot::bot::*;
// use crate::crates::

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let secure_data: serde_json::Value = get_private_data();
    let token: String = get_token(&secure_data);

    let _error = start_bot(token).await;
    log::info!("Stoping bot...");
    Ok(())
}
