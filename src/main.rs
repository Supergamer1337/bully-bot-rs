use serenity::{framework::StandardFramework, Client};

mod config;
mod handler;

use config::Config;
use handler::Handler;

#[tokio::main]
async fn main() {
    let config = Config::load();

    let framework = StandardFramework::new();
    let mut client = create_discord_client(framework, &config).await;

    println!("------------------");
    println!("Starting client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

async fn create_discord_client(framework: StandardFramework, config: &Config) -> Client {
    let handler = Handler::new(
        config.bully_chance,
        config.assets.clone(),
        config.cringe_channels.clone(),
        config.cringe_chance,
    );

    Client::builder(config.token.as_str(), config.intents)
        .event_handler(handler)
        .framework(framework)
        .await
        .expect("Error creating client")
}
