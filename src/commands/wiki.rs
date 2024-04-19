use poise::CreateReply;
use serenity::all::{CreateEmbed, CreateMessage, CreateSelectMenu, CreateSelectMenuOption};

use crate::{
    utilities::types::{QueryContainer, WikiQuery},
    Context, Error,
};

/// Shows Wikipedia search results.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Wiki",
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
            let title = "Search results for `".to_string() + &query + "`:";

            if data.1.is_empty() {
                let embed = CreateReply::default().embed(
                    CreateEmbed::new()
                        .title(title)
                        .description("No results for query."),
                );

                ctx.send(embed).await?;
                return Ok(());
            }

            // TODO: Make buttons/select menu
            let mut options = Vec::new();

            for (label, value) in data.1.iter().zip(data.3.iter()) {
                options.push(CreateSelectMenuOption::new(label, value));
            }

            let ctx_id = ctx.id();

            let menu = CreateSelectMenu::new(
                &ctx_id.to_string(),
                poise::serenity_prelude::CreateSelectMenuKind::String { options },
            )
            .max_values(1)
            .placeholder("Select a result from below to see summary.");

            let reply = CreateReply::default()
                .embed(
                    CreateEmbed::new()
                        .title(title)
                        .description(data.1.join("\n")),
                )
                .components(vec![poise::serenity_prelude::CreateActionRow::SelectMenu(
                    menu,
                )]);

            let handle = ctx.send(reply).await?;

            let refer = handle.into_message().await?;

            while let Some(interaction) =
                serenity::collector::ComponentInteractionCollector::new(ctx)
                    .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
                    .timeout(std::time::Duration::from_secs(3600 * 24))
                    .await
            {
                // TODO: Implement select interaction to show the summary of the selected article
                interaction.defer(ctx.http()).await?;

                match interaction.data.kind {
                    poise::serenity_prelude::ComponentInteractionDataKind::StringSelect {
                        values,
                    } => {
                        if let Some(value) = values.first() {
                            // interaction.channel_id.say(ctx.http(), "https://en.wikipedia.org/wiki/".to_owned() + value).await?; // debug

                            //TODO: Add summary fetching
                            let request = &ctx.data().reqwest;
                            let url = format!("https://en.wikipedia.org/w/api.php?format=json&action=query&prop=extracts&exintro&explaintext&redirects=1&titles={value}");

                            let get = request.get(url).send().await;

                            let res = match get {
                                Ok(res) => res.text().await,
                                Err(_) => {
                                    return Err("Failed to get data.".into());
                                }
                            };

                            let res = res.unwrap();

                            let data: QueryContainer = serde_json::from_str(&res).unwrap();
                            //info!("{:?}", data);

                            let data = data.query;

                            let embed = CreateEmbed::new()
                                .title(data.pages.title)
                                .description(data.pages.extract);

                            let message =
                                CreateMessage::new().embed(embed).reference_message(&refer);

                            ctx.channel_id().send_message(ctx.http(), message).await?;
                        }
                    }
                    _ => {}
                }
            }

            return Ok(());
        } else {
            return Err("Failed to deserialize the data from the Wikipedia API.".into());
        }
    } else {
        return Err("Wikipedia API data request failed.".into());
    }
}
