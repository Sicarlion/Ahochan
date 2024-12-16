use poise::serenity_prelude as serenity;
use serenity::{Context, CreateMessage, EditMessage, Message};
use crate::utils::read_line;
use crate::Data;
use std::fs;

#[derive(poise::ChoiceParameter)]
enum FetchOption {
    #[name = "all"]
    All,
    #[name = "last"]
    Last,
    #[name = "for"]
    For,
    #[name = "at"]
    At,
}

#[poise::command(slash_command)]
pub async fn fetch(
    ctx: poise::Context<'_, Data, anyhow::Error>,
    #[description = "Choose what to fetch"] option: FetchOption,
    #[description = "Line(s) to fetch ('for' and 'at')"]
    #[min = 1]
    lines: Option<usize>, // Optional parameter for number of lines
) -> Result<(), anyhow::Error> {
    match option {
        FetchOption::All => match fs::read_to_string("./log/msg.log") {
            Ok(log) => {
                //if ctx.author().id == 1257389104073674906 {
                ctx.say("> User have requested full message log file.")
                    .await?;

                let mut message = String::new();

                for line in log.lines() {
                    if message.len() + line.len() + 1 > 2000 {
                        let builder = CreateMessage::new().content(message);

                        ctx.author().dm(&ctx.http(), builder).await?;
                        message = String::new();
                    }

                    if !message.is_empty() {
                        message.push('\n');
                    }
                    message.push_str(line);
                }

                if !message.is_empty() {
                    let builder = CreateMessage::new().content(message);
                    ctx.author().dm(&ctx.http(), builder).await?;
                }
                //} else {
                //    ctx.say("> You lack the permission to do it.").await?;
                //}
            }
            Err(why) => {
                eprintln!("Error reading log file: {why:?}");
            }
        },
        FetchOption::Last => match read_line("./log/msg.log", 1) {
            Ok(line) => {
                ctx.say(line).await?;
            }
            Err(why) => {
                eprintln!("Error reading last line: {why:?}");
            }
        },
        FetchOption::At => {
            let num_lines = lines.unwrap_or(1);
            match read_line("./log/msg.log", num_lines) {
                Ok(lines) => {
                    ctx.say(format!("{}: {}", num_lines, lines)).await?;
                }
                Err(why) => {
                    eprintln!("Error reading lines: {why:?}");
                }
            }
        }
        FetchOption::For => {
            let mut builder = Vec::new();

            for i in 1..=lines.unwrap_or(1) {
                match read_line("./log/msg.log", i) {
                    Ok(line) => builder.push(line),
                    Err(why) => {
                        eprintln!("Error reading lines: {why:?}");
                    }
                }
            }

            builder.reverse();
            let formatted_output = builder.join("\n");
            ctx.say(formatted_output).await?;
        }
    }
    Ok(())
}

pub async fn fetch_last(ctx: &Context, msg: &Message) {
    let bot_msg = msg
        .channel_id
        .say(&ctx.http, "> **[FETCHER]** Fetching last message...")
        .await;

    let last_msg = match bot_msg {
        Ok(message) => message.id,
        Err(why) => {
            eprintln!("Error sending fetch message: {why:?}");
            return;
        }
    };

    match read_line("./log/msg.log", 1) {
        Ok(line) => {
            let builder = EditMessage::new().content(line);
            if let Err(why) = msg
                .channel_id
                .edit_message(&ctx.http, last_msg, builder)
                .await
            {
                eprintln!("Error editing message: {why:?}");
            }
        }
        Err(why) => {
            eprintln!("Error reading last line: {why:?}");
        }
    }
}
