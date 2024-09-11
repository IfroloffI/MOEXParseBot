pub mod bot {
    use crate::crates::requests::requests::*;
    use teloxide::prelude::*;

    #[tokio::main]
    pub async fn start_bot(token: String) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Starting bot...");
        
        let bot: Bot = Bot::new(token);
        
        teloxide::repl(bot, |bot: Bot, msg: Message| async move {

            let text: &str = &get_resp().await.unwrap();
            bot.send_message(msg.chat.id, text).await?;
            Ok(())
        })
        .await;

        Ok(())
    }
}
