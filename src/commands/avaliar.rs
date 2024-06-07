use serenity::all::{
    CommandInteraction, Context, CreateActionRow, CreateCommand, CreateInputText,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal, GuildId,
    InputTextStyle, RoleId,
};

pub async fn run(ctx: Context, command: CommandInteraction) {
    let role_id = RoleId::new(1212564013767073842);
    let guild_id = GuildId::new(1212488321419321355);
    if !command
        .user
        .has_role(&ctx.http, guild_id, role_id)
        .await
        .unwrap()
    {
        let message = CreateInteractionResponseMessage::new()
            .content("> Você não é um cliente para avaliar nosso atendimento.");

        let response = CreateInteractionResponse::Message(message);

        return command.create_response(&ctx.http, response).await.unwrap();
    }

    let input_text = CreateInputText::new(
        InputTextStyle::Paragraph,
        "Avaliação",
        "avaliar-modal-text-input",
    );

    let modal_component = CreateActionRow::InputText(input_text);

    let modal =
        CreateModal::new("avaliar-modal", "Avaliar atendimento").components(vec![modal_component]);

    let response = CreateInteractionResponse::Modal(modal);
    command.create_response(&ctx.http, response).await.unwrap()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("avaliar").description("Avaliar atendimento.")
}
