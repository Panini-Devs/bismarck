use poise::CreateReply;
use serenity::all::CreateEmbed;

use crate::{utilities::types::WikiQuery, Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    category = "Utilities",
    user_cooldown = 5,
    required_bot_permissions = "SEND_MESSAGES"
)]
pub async fn wiki(
    ctx: Context<'_>,
    #[description = "The search query"] query: String,
) -> Result<(), Error> {
    let request = &ctx.data().reqwest;

    let params = [
        ("action", "opensearch"),
        ("format", "json"),
        ("namespace", "0"),
        ("search", &query),
        ("limit", "3"),
    ];

    let url =
        reqwest::Url::parse_with_params("https://en.wikipedia.org/w/api.php", params).unwrap();

    if let Ok(res) = request.get(url).send().await {
        let data = res.json::<WikiQuery>().await;

        if let Ok(data) = data {
            let embed = CreateReply::default().embed(
                CreateEmbed::new()
                    .title(data.0)
                    .description(data.1.join("\n")),
            );

            ctx.send(embed).await?;

            return Ok(());
        } else {
            return Err("Failed to deserialize the data from the Wikipedia API.".into());
        }
    } else {
        return Err("Wikipedia API data request failed.".into());
    }
}
