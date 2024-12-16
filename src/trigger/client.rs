use poise::serenity_prelude as serenity;
use serenity::{ActivityData, Context};

pub async fn on_ready(ctx: Context, client: poise::serenity_prelude::Ready) {
    ctx.shard
        .set_activity(Some(ActivityData::watching("rule34.xxx")));
    println!("[LOG] {} is connected.", client.user.name);
}
