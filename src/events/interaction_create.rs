use super::buttons;
use super::commands;
use super::menus;
use super::modals;

use serenity::all::{Context, Interaction};

pub async fn run(ctx: Context, interaction: Interaction) {
    match interaction {
        Interaction::Command(command) => match command.data.name.as_str() {
            "setup" => commands::setup::run(ctx, command).await,
            "avaliar" => commands::avaliar::run(ctx, command).await,
            _ => println!("❌ - Command not found!"),
        },
        Interaction::Component(component) => match component.data.custom_id.as_str() {
            "ticket-select-category" => menus::ticket::run(ctx, component).await,
            "ticket-button-close" => buttons::close_ticket::run(ctx, component).await,
            "ticket-button-add-member" => buttons::add_ticket::run(ctx, component).await,
            _ => println!("❌ - Component not found!"),
        },
        Interaction::Modal(component) => match component.data.custom_id.as_str() {
            "avaliar-modal" => modals::avaliar::run(ctx, component).await,
            _ => println!("❌ - Modal not found!"),
        },
        _ => println!("🔘 - Some other interaction detected!"),
    }
}
