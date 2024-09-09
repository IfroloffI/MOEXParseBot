use mongodb::{ 
    bson::{Document, doc},
    Client,
    Collection
};
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    const token: &str = "935459293:AAHI7XXWVeYehEDR_DCUo8uT4zDOoBX0jCs";

    let uri = "mongodb://127.0.0.1:27017";
    let client = Client::with_uri_str(uri).await?;
    // Get a handle on the movies collection
    let database = client.database("local");
    let my_coll: Collection<Document> = database.collection("startup_log");
    // Find a movie based on the title value
    let value = my_coll.find(doc! {}).await?;
    //println!("Found a value: {:#?}", value);

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
