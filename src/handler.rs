use crate::Bot;

use poise::serenity_prelude as serenity;
use serenity::{Context, EventHandler, Message};
use crate::trigger::message::on_message;
use crate::trigger::client::on_ready;

#[serenity::async_trait()]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, client: poise::serenity_prelude::Ready) {
        on_ready(ctx, client).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        on_message(ctx, msg).await;
    }
}
