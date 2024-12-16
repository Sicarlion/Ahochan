use crate::Data;

#[poise::command(slash_command)]
pub async fn hello(ctx: poise::Context<'_, Data, anyhow::Error>) -> Result<(), anyhow::Error> {
    ctx.say("Hello, world! ヾ(＾∇＾)").await?;
    Ok(())
}
