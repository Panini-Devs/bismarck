use crate::{Context, Error};

/// Multiplies two numbers.
#[poise::command(prefix_command, slash_command)]
pub async fn multiply(
    context: Context<'_>,
    #[description = "One number to be multiplied"] one: Option<f64>,
    #[description = "Another number to be multiplied"] two: Option<f64>
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
#[poise::command(prefix_command, slash_command)]
pub async fn add(
    context: Context<'_>,
    #[description = "One number to be added"] one: Option<f64>,
    #[description = "Another number to be added"] two: Option<f64>
) -> Result<(), Error> {

    let one = one.unwrap_or(1.0);
    let two = two.unwrap_or(1.0);

    let product = one + two;

    let _ = context
        .say(format!("{} + {} = {}", one, two, product))
        .await;

    Ok(())
}