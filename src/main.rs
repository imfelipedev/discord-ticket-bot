use dotenv::dotenv;
use serenity::{all::GatewayIntents, Client};
use std::env;

use bot_ticket::handlers::event::Handler;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("❌ - Token not found!");

    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("❌ - Error creating client!");

    if let Err(err) = client.start().await {
        println!("❌ - Client error: {err:?}");
    }
}
