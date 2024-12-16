use poise::serenity_prelude as serenity;
use serenity::{ChannelId, Context, CreateEmbed, CreateEmbedAuthor, CreateMessage, Message};

pub async fn pin(ctx: &Context, msg: &Message) {
    let replies;
    if let Some(reply) = &msg.referenced_message {
        replies = reply;
    } else {
        msg.channel_id
            .say(&ctx.http, "> Uhh.. Whom to pin? (Reply to the message..)")
            .await
            .expect("Cannot send message.");
        return;
    }

    let pinboard = ChannelId::new(1317827618933968896);
    let mut embed_builder = CreateEmbed::new()
        .author(
            CreateEmbedAuthor::new(replies.author.display_name())
                .icon_url(replies.author.avatar_url().unwrap_or("".to_string())),
        )
        .description(format!(
            "{}\n\n**Posted on:** {}",
            &replies.content,
            &replies.link()
        ));

    if let Some(attachment) = replies.attachments.first() {
        embed_builder = embed_builder.image(&attachment.url);
    }

    let builder = CreateMessage::new().embed(embed_builder);

    if let Err(why) = pinboard.send_message(&ctx.http, builder).await {
        eprintln!("Error sending message: {why:?}");
    };
}
