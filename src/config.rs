use dotenv::dotenv;
use std::{env, fs, io, path::PathBuf};

use serenity::prelude::GatewayIntents;

pub struct Config {
    pub token: String,
    pub bully_chance: f64,
    pub assets: Vec<PathBuf>,
    pub cringe_channels: Vec<u64>,
    pub cringe_chance: f64,
    pub intents: GatewayIntents,
}

impl Config {
    pub fn load() -> Self {
        dotenv().ok();

        let files = read_bully_files();
        let (token, intents) = get_token_and_intents();
        let bully_chance = get_bully_chance();
        let cringe_channels = get_cringe_channels();
        let cringe_chance = get_cringe_chance();

        Self {
            token,
            bully_chance,
            assets: files,
            cringe_channels,
            cringe_chance,
            intents,
        }
    }
}

fn read_bully_files() -> Vec<PathBuf> {
    let mut errors: Vec<io::Error> = Vec::new();
    let files: Vec<PathBuf> = fs::read_dir("assets")
        .expect("Failed to read assets folder")
        .filter_map(|f| f.map_err(|e| errors.push(e)).ok())
        .map(|f| f.path())
        .collect();

    print_errors_and_files(&errors, &files);

    files
}

fn print_errors_and_files(errors: &Vec<io::Error>, files: &Vec<PathBuf>) {
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
        if let Some(file_name) = file.file_name().and_then(|f| f.to_str()) {
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
