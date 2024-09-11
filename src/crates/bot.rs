pub mod bot {
    // use crate::crates::request_data::request_data::*;
    use teloxide::prelude::*;

    pub async fn start_bot(token: String) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Starting bot...");
        let bot: Bot = Bot::new(token);
        
        teloxide::repl(bot, |bot: Bot, msg: Message| async move {

            let text: &str = "hello";
            // println!("{}", get_resp().await.unwrap());
            bot.send_message(msg.chat.id, text).await?;
            Ok(())
        })
        .await;

        Ok(())
    }
}