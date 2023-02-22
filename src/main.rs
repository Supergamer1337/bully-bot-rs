use dotenv::dotenv;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serenity::utils::MessageBuilder;
use std::fs::DirEntry;
use std::{env, fs};

use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler {
    bully_chance: f64,
    assets: Vec<DirEntry>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let num: f64 = thread_rng().gen();
        if num < self.bully_chance {
            let video = self.assets.choose(&mut thread_rng()).unwrap();
            let video_path = video.path();

            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.reference_message(&msg);
                    m.add_file(&video_path);
                    m
                })
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new();

    let files: Vec<DirEntry> = fs::read_dir("assets")
        .unwrap()
        .map(|f| f.unwrap())
        .collect();
    println!("{:?}", files);

    let token = env::var("DISCORD_TOKEN").expect("No token provided");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler {
            bully_chance: env::var("BULLY_CHANCE")
                .expect("No bully chance provided")
                .parse()
                .expect("Bully chance is not a number"),
            assets: files,
        })
        .framework(framework)
        .await
        .expect("Error creating client");

    println!("Starting client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
