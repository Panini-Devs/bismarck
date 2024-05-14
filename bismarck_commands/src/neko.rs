use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateEmbedFooter};

use bismarck_core::{context::Context, error::Error, types::Items};

/// Sends a random Neko image.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Neko",
    user_cooldown = 5,
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn neko(context: Context<'_>) -> Result<(), Error> {
    let request = &context.data().reqwest;

    let params = [("rating", "safe"), ("limit", "1")];

    let url = reqwest::Url::parse_with_params("https://api.nekosapi.com/v3/images/random", params)
        .unwrap();

    let res = request.get(url).send().await;

    let res = match res {
        Ok(res) => res,
        Err(_) => {
            context.reply("Failed to get image.").await?;
            return Ok(());
        }
    };

    //info!("{}", res.text().await?);

    let data = res.json::<Items>().await;

    let data = match data {
        Ok(data) => data,
        Err(_) => {
            context.reply("Failed to get image.").await?;
            return Ok(());
        }
    };

    let image_url = data.items[0].image_url.clone();
    let id = data.items[0].id.clone().to_string();

    let embed = CreateReply::default().embed(
        CreateEmbed::new()
            .image(image_url)
            .title("Random Neko Image!")
            .description(id)
            .colour(0xff0055)
            .footer(CreateEmbedFooter::new("Powered by https://nekosapi.com")),
    );

    context.send(embed).await?;

    Ok(())
}
