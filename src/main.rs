use dotenv::dotenv;
use serenity::{framework::StandardFramework, model::prelude::*, Client};
use std::{
    env,
    fs::{self, DirEntry},
    io,
};

mod handler;

use handler::Handler;

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
    let handler = Handler::new(
        get_bully_chance(),
        files,
        get_cringe_channels(),
        get_cringe_chance(),
    );

    Client::builder(token, intents)
        .event_handler(handler)
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

fn get_cringe_channels() -> Vec<u64> {
    let cringe_channels = match env::var("CRINGE_CHANNELS") {
        Ok(channels) => channels,
        Err(_) => {
            println!("------------------");
            println!("No cringe channels provided, skipping cringe messages");
            return Vec::new();
        }
    };

    if cringe_channels.is_empty() {
        println!("------------------");
        println!("No cringe channels provided, skipping cringe messages");
        return Vec::new();
    }

    let channels: Vec<u64> = cringe_channels
        .split(",")
        .map(|s| s.parse().expect("Cringe channel is not a number"))
        .collect();

    channels
}

fn get_cringe_chance() -> f64 {
    let cringe_chance = match env::var("CRINGE_CHANCE") {
        Ok(chance) => chance,
        Err(_) => {
            println!("------------------");
            println!("No cringe chance provided, skipping cringe messages");
            return 0.0;
        }
    };

    cringe_chance
        .parse()
        .expect("Cringe chance is not a float number")
}
