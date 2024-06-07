use serenity::all::{
    ActionRow, ActionRowComponent, ChannelId, Colour, Context, CreateEmbed, CreateEmbedAuthor,
    CreateEmbedFooter, CreateMessage, EditInteractionResponse, ModalInteraction, Timestamp,
};

fn get_input(action_rows: &Vec<ActionRow>, id: &str) -> Option<String> {
    for row in action_rows {
        for component in &row.components {
            match component {
                ActionRowComponent::InputText(input) => {
                    if input.custom_id == id {
                        return input.value.clone();
                    }
                }
                _ => {}
            }
        }
    }

    None
}

pub async fn run(ctx: Context, component: ModalInteraction) {
    component.defer_ephemeral(&ctx.http).await.unwrap();

    let input = match get_input(&component.data.components, "avaliar-modal-text-input") {
        Some(input) => input,
        None => {
            let response = EditInteractionResponse::new().content("> Esse input é invalido.");

            component.edit_response(&ctx.http, response).await.unwrap();
            return;
        }
    };

    let guild = component.guild_id.unwrap();

    let guild_channels = guild.channels(&ctx.http).await.unwrap();

    let channel_id = ChannelId::new(1212574968789999669);

    let channel = guild_channels.get(&channel_id).unwrap();

    let member_avatar = component.user.avatar_url().unwrap();

    let author = CreateEmbedAuthor::new(&component.user.name).icon_url(member_avatar);

    let description = format!("> {}", input);

    let footer = CreateEmbedFooter::new("Quantum Codes - Ticket");

    let timestamp = Timestamp::now();

    let embed = CreateEmbed::default()
        .author(author)
        .description(description)
        .footer(footer)
        .timestamp(timestamp)
        .colour(Colour::LIGHT_GREY);

    let message = CreateMessage::new().add_embed(embed);

    channel.send_message(&ctx.http, message).await.unwrap();

    let response = EditInteractionResponse::new()
        .content("> Avaliação realizada com sucesso, a equipe Quantum Codes agradece.");

    component.edit_response(&ctx.http, response).await.unwrap();
}
