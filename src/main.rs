use dotenv::dotenv;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::fs::DirEntry;
use std::io;
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

        if !message_is_meme(&msg) {
            return;
        }

        let num: f64 = thread_rng().gen();
        if num < self.bully_chance {
            send_bully_message(&ctx, &msg, &self).await;
        }
    }
}

fn message_is_meme(msg: &Message) -> bool {
    if msg.embeds.len() > 0
        || msg.content.contains("https://")
        || msg.content.contains("http://")
        || msg.attachments.len() > 0
    {
        true
    } else {
        false
    }
}

async fn send_bully_message(ctx: &Context, msg: &Message, handler: &Handler) {
    let video = match handler.assets.choose(&mut thread_rng()) {
        Some(file) => file.path(),
        None => {
            println!("Failed to choose a bully file. Skipping message.");
            return;
        }
    };

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
    let mut client = create_discord_client(framework).await;

    println!("------------------");
    println!("Starting client");
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

async fn create_discord_client(framework: StandardFramework) -> Client {
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
    let mut errors: Vec<io::Error> = Vec::new();
    let files: Vec<DirEntry> = fs::read_dir("assets")
        .expect("Failed to read assets folder")
        .filter_map(|f| f.map_err(|e| errors.push(e)).ok())
        .collect();

    print_errors_and_files(&errors, &files);

    files
}

fn print_errors_and_files(errors: &Vec<io::Error>, files: &Vec<DirEntry>) {
    if errors.len() > 0 {
        println!(
            "Errors reading {} files, skipping them with the following errors:",
            errors.len()
        );
        for error in errors {
            println!("Error reading file: {:?}", error);
            println!("------------------")
        }
    }

    println!("Found {} bully files:", files.len());
    for file in files {
        if let Some(file_name) = file.file_name().to_str() {
            println!("{}", file_name);
        } else {
            println!("Unknown file name");
        }
    }
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
