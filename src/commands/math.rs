use crate::{Context, Error};

/// Multiplies two numbers.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Math",
    required_permissions = "SEND_MESSAGES"
)]
pub async fn multiply(
    context: Context<'_>,
    #[description = "One number to be multiplied"] one: Option<f64>,
    #[description = "Another number to be multiplied"] two: Option<f64>,
) -> Result<(), Error> {
    let one = one.unwrap_or(1.0);
    let two = two.unwrap_or(1.0);

    let product = one * two;

    let _ = context
        .say(format!("{} * {} = {}", one, two, product))
        .await;

    Ok(())
}

/// Adds two numbers.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Math",
    required_permissions = "SEND_MESSAGES"
)]
pub async fn add(
    context: Context<'_>,
    #[description = "One number to be added"] one: Option<f64>,
    #[description = "Another number to be added"] two: Option<f64>,
) -> Result<(), Error> {
    let one = one.unwrap_or(1.0);
    let two = two.unwrap_or(1.0);

    let product = one + two;

    let _ = context
        .say(format!("{} + {} = {}", one, two, product))
        .await;

    Ok(())
}

/// Subtracts two numbers.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Math",
    required_permissions = "SEND_MESSAGES"
)]
pub async fn divide(
    context: Context<'_>,
    #[description = "Number to be divided"] dividend: Option<f64>,
    #[description = "A number to divide One"] divisor: Option<f64>,
) -> Result<(), Error> {
    let one = dividend.unwrap_or(1.0);
    let two = divisor.unwrap_or(1.0);

    if two == 0.0 {
        let _ = context.say("Divisor cannot be 0!").await;
        return Ok(());
    }

    let quotient = one / two;

    let _ = context
        .say(format!("{} / {} = {}", one, two, quotient))
        .await;

    Ok(())
}

/// Subtracts two numbers.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Math",
    required_permissions = "SEND_MESSAGES"
)]
pub async fn subtract(
    context: Context<'_>,
    #[description = "One number to be subtracted"] one: Option<f64>,
    #[description = "Another number to be subtracted"] two: Option<f64>,
) -> Result<(), Error> {
    let one = one.unwrap_or(1.0);
    let two = two.unwrap_or(1.0);

    let difference = one - two;

    let _ = context
        .say(format!("{} - {} = {}", one, two, difference))
        .await;

    Ok(())
}
