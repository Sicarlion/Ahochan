use crate::Data;
use std::fs::{self, File};

#[poise::command(slash_command)]
pub async fn clean(ctx: poise::Context<'_, Data, anyhow::Error>) -> Result<(), anyhow::Error> {
    if let Err(why) = fs::remove_dir_all("./log") {
        eprintln!("Error why cleaning: {why:?}");
    }

    if fs::create_dir("./log/").is_ok() {
        println!("Log dir created.");
    } else {
        println!("Log dir already exists. Somehow.");
    }

    if File::create_new("./log/msg.log").is_ok() {
        println!("Log file created.");
    } else {
        println!("Log file already exists. Somehow.");
    }
    ctx.say("> Database cleaned.").await?;
    Ok(())
}
