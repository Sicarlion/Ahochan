use crate::Bot;
use crate::log_message;
use crate::pin;
use crate::fetch_last;
use crate::check_newspaper;
use crate::serenity::{Message, ActivityData, Context, EventHandler};

#[serenity::async_trait()]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, client: poise::serenity_prelude::Ready) {
        ctx.shard
            .set_activity(Some(ActivityData::watching("rule34.xxx")));
        println!("[LOG] {} is connected.", client.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
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
}
