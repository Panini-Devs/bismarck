use poise::samples::HelpConfiguration;

use crate::{Context, Error};
/// Show help message
#[poise::command(prefix_command, track_edits, category = "Utility")]
async fn help(
    context: Context<'_>,
    #[description = "Command to get help for"]
    #[rest]
    mut command: Option<String>,
) -> Result<(), Error> {
    // This makes it possible to just make `help` a subcommand of any command
    // `/fruit help` turns into `/help fruit`
    // `/fruit help apple` turns into `/help fruit apple`
    if context.invoked_command_name() != "help" {
        command = match command {
            Some(c) => Some(format!("{} {}", context.invoked_command_name(), c)),
            None => Some(context.invoked_command_name().to_string()),
        };
    }
    let extra_text_at_bottom = "\
Type `?help command` for more info on a command.
You can edit your `?help` message to the bot and the bot will edit its response.";

    let config = HelpConfiguration {
        show_subcommands: true,
        show_context_menu_commands: true,
        ephemeral: true,
        extra_text_at_bottom,

        ..Default::default()
    };
    poise::builtins::help(context, command.as_deref(), config).await?;
    Ok(())
}
