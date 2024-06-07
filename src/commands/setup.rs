use serenity::all::{
    Colour, CommandInteraction, Context, CreateActionRow, CreateCommand, CreateEmbed,
    CreateEmbedFooter, CreateMessage, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption, EditInteractionResponse, Timestamp,
};

pub async fn run(ctx: Context, command: CommandInteraction) {
    command.defer_ephemeral(&ctx.http).await.unwrap();

    let timestamp = Timestamp::now();

    let footer = CreateEmbedFooter::new("Quantum Codes - Ticket");

    let ticket_embed = CreateEmbed::default()
        .description("🏷️ Central de atendimento\n> Para solicitar suporte, selecione um das opções abaixo.\n\n⏰ - Horário de atendimento:\n- **Segunda a sexta**, das **08:00 as 18:00**")
        .image("https://cdn.discordapp.com/attachments/1206088180978425937/1215016501996814386/standard_2.gif?ex=65fb374b&is=65e8c24b&hm=3598075bb9e3f3e4315e3afa0f5f7f168a3f220622ea07ca7e0460521cfd7ee0&")
        .footer(footer)
        .timestamp(timestamp)
        .colour(Colour::LIGHT_GREY);

    let select_menu_order_option =
        CreateSelectMenuOption::new("Encomenda", "Solicitar um sistema exclusivo.");

    let select_menu_suport_option =
        CreateSelectMenuOption::new("Suporte", "Iniciar suporte privado.");

    let select_menu_kind = CreateSelectMenuKind::String {
        options: vec![select_menu_order_option, select_menu_suport_option],
    };

    let select_menu = CreateSelectMenu::new("ticket-select-category", select_menu_kind)
        .placeholder("Selecione uma opção.");

    let row = CreateActionRow::SelectMenu(select_menu);

    let components = vec![row];

    let ticket_message = CreateMessage::new()
        .add_embed(ticket_embed)
        .components(components);

    command
        .channel_id
        .send_message(&ctx.http, ticket_message)
        .await
        .unwrap();

    let data = EditInteractionResponse::new().content("Setup finalizado com sucesso.");

    command.edit_response(&ctx.http, data).await.unwrap();
}

pub fn register() -> CreateCommand {
    CreateCommand::new("setup").description("Iniciar setup do ticket.")
}
