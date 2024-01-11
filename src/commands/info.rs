

#[poise::command(prefix_command, slash_command)]
#[description = "Get information about the bot."]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This is the info command!").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
#[description = "Get information about the user."]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}