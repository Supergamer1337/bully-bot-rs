use dotenv::dotenv;
use rand::{thread_rng, Rng};
use std::env;

use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler {
    bully_chance: f64,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let num: f64 = thread_rng().gen();
        if num < self.bully_chance {
            if let Err(why) = msg.reply(&ctx.http, "You're being bullied").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new();

    let token = env::var("DISCORD_TOKEN").expect("No token provided");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            bully_chance: env::var("BULLY_CHANCE")
                .expect("No bully chance provided")
                .parse()
                .expect("Bully chance is not a number"),
        })
        .framework(framework)
        .await
        .expect("Error creating client");

    println!("Starting client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
