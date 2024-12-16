use poise::serenity_prelude as serenity;
use serenity::{Context, Message};
use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn log_message(msg: &Message) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open("./log/msg.log")?;

    let log_entry = format!(
        "{} on <#{}>: {}\n",
        msg.author.display_name(),
        msg.channel_id,
        msg.content
    );

    file.write_all(log_entry.as_bytes())
}

pub async fn check_newspaper(ctx: &Context, msg: &Message) {
    if msg.channel_id == 1238505179418988638 {
        if msg.attachments.len() == 0
            && !msg.content.contains("http")
            && !msg.content.contains("https")
        {
            if let Err(why) = msg.delete(&ctx).await {
                eprintln!("[ERR] Cannot delete message. {why:?}");
            }
        }
    }
}
