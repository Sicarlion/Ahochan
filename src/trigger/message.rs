use crate::bot::{check_newspaper, log_message};
use crate::commands::fetch::fetch_last;
use crate::commands::pin::pin;

use poise::serenity_prelude as serenity;
use serenity::{Context, Message};

pub async fn on_message(ctx: Context, msg: Message) {
    let commands = ["clip", "pin"];

    if msg.author.bot {
        return;
    }

    check_newspaper(&ctx, &msg).await;

    if msg.content == commands[0] {
        fetch_last(&ctx, &msg).await;
    } else if msg.content == commands[1] {
        pin(&ctx, &msg).await;
    } else {
        log_message(&msg).expect("Failed to log message.");
    }
}
