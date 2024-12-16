use crate::crates::requests::*;
use teloxide::prelude::*;

#[tokio::main]
pub async fn start_bot(token: String) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Starting bot...");
        
    let bot: Bot = Bot::new(token);
        
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let moex_data = get_moex_data().await.unwrap();
        if moex_data.securities.data.is_empty() {
            bot.send_message(msg.chat.id, "Нет данных о ценах валют").await?;
        } else {
            let text = moex_data.securities.data.iter().map(|security| security.join(", ")).collect::<Vec<_>>().join("\n");
            bot.send_message(msg.chat.id, text).await?;
        }
        Ok(())
    })
 .await;

    Ok(())
}