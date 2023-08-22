use rand::{seq::SliceRandom, Rng};
use serenity::{model::prelude::*, prelude::*};
use std::path::PathBuf;

pub struct Handler {
    bully_chance: f64,
    cringe_channels: Vec<u64>,
    cringe_chance: f64,
    assets: Vec<PathBuf>,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if !message_is_meme(&msg) {
            return;
        }

        let num: f64 = rand::thread_rng().gen();
        if self.cringe_channels.contains(&msg.channel_id.as_u64()) {
            if num < self.cringe_chance {
                return;
            }

            Self::send_cringe_message(&ctx, &msg).await;
            return;
        }

        if num < self.bully_chance {
            self.send_bully_message(&ctx, &msg).await;
        }
    }
}

impl Handler {
    pub fn new(
        bully_chance: f64,
        assets: Vec<PathBuf>,
        cringe_channels: Vec<u64>,
        cringe_chance: f64,
    ) -> Self {
        Self {
            bully_chance,
            assets,
            cringe_channels,
            cringe_chance,
        }
    }

    async fn send_bully_message(&self, ctx: &Context, msg: &Message) {
        let video = match self.assets.choose(&mut rand::thread_rng()) {
            Some(file) => file,
            None => {
                println!("Failed to choose a bully file. Skipping message.");
                return;
            }
        };

        if let Err(why) = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.reference_message(msg);
                m.add_file(video);
                m
            })
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn send_cringe_message(ctx: &Context, msg: &Message) {
        if let Err(why) = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.reference_message(msg);
                m.content("Honestly, this is cringe.");
                m
            })
            .await
        {
            println!("Error sending message: {:?}", why);
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
