use dotenv::dotenv;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
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
            send_bully_message(&ctx, &msg, &self).await;
        }
    }
}

async fn send_bully_message(ctx: &Context, msg: &Message, handler: &Handler) {
    let video = handler.assets.choose(&mut thread_rng()).unwrap().path();

    if let Err(why) = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.reference_message(msg);
            m.add_file(&video);
            m
        })
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new();
    let mut client = create_client(framework).await;

    println!("------------------");
    println!("Starting client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

async fn create_client<'a>(framework: StandardFramework) -> Client {
    let files = read_bully_files();
    let (token, intents) = get_token_and_intents();
    Client::builder(token, intents)
        .event_handler(Handler {
            bully_chance: get_bully_chance(),
            assets: files,
        })
        .framework(framework)
        .await
        .expect("Error creating client")
}

fn read_bully_files() -> Vec<DirEntry> {
    let files: Vec<DirEntry> = fs::read_dir("assets")
        .unwrap()
        .map(|f| f.unwrap())
        .collect();

    println!("Found {} bully files:", files.len());
    for file in &files {
        println!("{}", file.file_name().to_str().unwrap());
    }

    files
}

fn get_token_and_intents() -> (String, GatewayIntents) {
    let token = env::var("DISCORD_TOKEN").expect("No token provided");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    (token, intents)
}

fn get_bully_chance() -> f64 {
    let bully_chance = env::var("BULLY_CHANCE").expect("No bully chance provided");
    bully_chance
        .parse()
        .expect("Bully chance is not a float number")
}
