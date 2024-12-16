use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions};
use serenity::{
    ActivityData, ChannelId, Client, Context, CreateEmbed, CreateEmbedAuthor, CreateMessage,
    EditMessage, EventHandler, GatewayIntents, Message,
};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};

struct Data {}
struct Bot {}

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

fn log_message(msg: &Message) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open("./log/msg.log")?;

    let log_entry = format!(
        "{} on <#{}>: {}\n",
        msg.author.display_name(),
        msg.channel_id,
        msg.content
    );

    file.write_all(log_entry.as_bytes())
}

async fn pin(ctx: &Context, msg: &Message) {
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

async fn fetch_last(ctx: &Context, msg: &Message) {
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

fn read_line(file_path: &str, line: usize) -> io::Result<String> {
    let request = line - 1;
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();
    let mut lines = Vec::new();

    let file_size = reader.seek(SeekFrom::End(0))?;
    let mut position = file_size;

    while position > 0 {
        position -= 1;
        reader.seek(SeekFrom::Start(position))?;
        let mut byte = [0; 1];
        reader.read_exact(&mut byte)?;

        if byte[0] == b'\n' && !buffer.is_empty() {
            buffer.reverse();
            lines.push(String::from_utf8(buffer.clone()).unwrap_or_default());
            buffer.clear();

            if lines.len() > request {
                break;
            }
        } else {
            buffer.push(byte[0]);
        }
    }

    if !buffer.is_empty() {
        buffer.reverse();
        lines.push(String::from_utf8(buffer.clone()).unwrap_or_default());

        buffer.clear();
    }

    if request < lines.len() {
        println!("{:?}", lines);
        Ok(lines[request].clone())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Requested line number is out of bounds",
        ))
    }
}

async fn check_newspaper(ctx: &Context, msg: &Message) {
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

#[poise::command(slash_command)]
async fn hello(ctx: poise::Context<'_, Data, anyhow::Error>) -> Result<(), anyhow::Error> {
    ctx.say("Hello, world! ヾ(＾∇＾)").await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn clean(ctx: poise::Context<'_, Data, anyhow::Error>) -> Result<(), anyhow::Error> {
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
async fn fetch(
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
