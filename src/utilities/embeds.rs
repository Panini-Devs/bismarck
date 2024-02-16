use chrono::NaiveDateTime;
use serenity::{
    all::{colours::css, User},
    builder::{CreateEmbed, CreateEmbedAuthor},
    model::Colour,
};
use std::fmt::Write;

// Modified from wakalaka-rs
pub fn warnings_command_embed(
    user: &User,
    uuids: &[&String],
    moderator_ids: &[&i64],
    reasons: &[&String],
    dates: &[&NaiveDateTime],
) -> CreateEmbed {
    //  |(PFP) {user_name}                |
    //  | ID      | Moderator | Reason    |
    //  |---------|-----------|-----------|
    //  | <uuid1> | <@{id1}>  | {reason1} |
    //  | <uuid2> | <@{id2}>  | {reason2} |
    //  | <uuid3> | <@{id3}>  | {reason3} |
    //  ===================================
    //  | ID      | Date      |
    //  |---------|-----------|
    //  | <uuid1> | {date1}   |
    //  | <uuid2> | {date2}   |
    //  | <uuid3> | {date3}   |

    let (user_name, user_avatar_url) = (
        &user.name,
        user.avatar_url().unwrap_or(user.default_avatar_url()),
    );

    let embed_author = CreateEmbedAuthor::new(user_name).icon_url(user_avatar_url);

    let mut id_field = String::new();
    let mut moderator_field = String::new();
    let mut reason_field = String::new();
    let mut date_field = String::new();
    for (((uuid, moderator_id), reason), date) in uuids
        .iter()
        .zip(moderator_ids.iter())
        .zip(reasons.iter())
        .zip(dates.iter())
    {
        writeln!(id_field, "{uuid}").unwrap();
        writeln!(moderator_field, "<@{moderator_id}>").unwrap();
        writeln!(reason_field, "{reason}").unwrap();
        writeln!(date_field, "{date}").unwrap();
    }

    let embed_fields = vec![
        ("ID", id_field.clone(), true),
        ("Moderator", moderator_field, true),
        ("Reason", reason_field, true),
        ("\u{200B}", "\u{200B}".to_owned(), false),
        ("ID", id_field, true),
        ("Date", date_field, true),
    ];

    CreateEmbed::default()
        .author(embed_author)
        .fields(embed_fields)
}

pub fn error_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(message.to_string())
        .colour(css::DANGER)
}

pub fn info_message_embed(message: &String) -> CreateEmbed {
    CreateEmbed::default()
        .description(message.to_string())
        .colour(Colour::BLUE)
}
