mod utils;
use crate::utils::json_processing::json::*;

use mongodb::{ 
    bson::{Document, doc},
    Client,
    Collection
};
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let secure_data: serde_json::Value = get_private_data();
    let token: String = get_token(&secure_data);

    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");
    let bot: Bot = Bot::new(token);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;

    Ok(())
}
