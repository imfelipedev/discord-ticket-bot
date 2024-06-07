use super::commands;
use serenity::all::CreateCommand;

pub fn register_commands() -> Vec<CreateCommand> {
    vec![commands::setup::register(), commands::avaliar::register()]
}
