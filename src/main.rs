mod bot;
mod commands;
mod handler;
mod trigger;
mod utils;

use commands::clean::clean;
use commands::fetch::fetch;
use commands::hello::hello;

use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions};
use serenity::{Client, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use std::fs::{self, File};

pub struct Data {}
pub struct Bot {}

#[shuttle_runtime::main]
async fn ahochan(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    if fs::create_dir("./log/").is_ok() {
        println!("Log dir created.");
    } else {
        println!("Log dir already exists.");
    }
    if File::create_new("./log/msg.log").is_ok() {
        println!("Log file created.");
    } else {
        println!("Log file already exists.");
    }
    let token = secret_store
        .get("DISCORD_TOKEN")
        .expect("[ERR] Discord token missing in secrets.");

    let intents = GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES;

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![hello(), fetch(), clean()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    1229664609141653514.into(),
                )
                .await?;
                Ok(Data {})
            })
        })
        .build();

    let client = Client::builder(token, intents)
        .event_handler(Bot {})
        .framework(framework)
        .await
        .ok()
        .unwrap();

    // Return the bot as a Shuttle service
    Ok(client.into())
}
