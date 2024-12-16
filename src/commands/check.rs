use crate::Data;
use poise::serenity_prelude as serenity;
use rand::Rng;
use serenity::Member;

#[derive(poise::ChoiceParameter)]
enum CheckOption {
    #[name = "femboy"]
    Femboy,
}

#[poise::command(slash_command)]
pub async fn check(
    ctx: poise::Context<'_, Data, anyhow::Error>,
    #[description = "Choose what to check"] option: CheckOption,
    #[description = "Whom to check?"] person: Member,
) -> Result<(), anyhow::Error> {
    match option {
        CheckOption::Femboy => {
            let femboy;

            if person.user.id == 850942765722107904 {
                femboy = rand::thread_rng().gen_range(75..=100);
            } else {
                femboy = rand::thread_rng().gen_range(0..=100);
            }

            ctx.say(format!("> <@{}> is {}% femboy.", person.user.id, femboy)).await.unwrap();
        }
    }
    Ok(())
}
