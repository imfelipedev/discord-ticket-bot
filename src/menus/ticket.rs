use serenity::all::{
    ButtonStyle, ChannelId, ChannelType, Colour, ComponentInteraction, Context, CreateActionRow,
    CreateButton, CreateChannel, CreateEmbed, CreateEmbedFooter, CreateMessage,
    EditInteractionResponse, GuildChannel, PermissionOverwrite, PermissionOverwriteType,
    Permissions, RoleId, Timestamp,
};

struct Ticket {
    ctx: Context,
    component: ComponentInteraction,
}

impl Ticket {
    pub fn new(ctx: Context, component: ComponentInteraction) -> Ticket {
        Ticket { ctx, component }
    }

    pub async fn create_channel(&self) -> GuildChannel {
        let guild = self.component.guild_id.unwrap();

        let guild_id = guild.get();

        let guild_role = RoleId::new(guild_id);

        let permissions = vec![
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny: Permissions::VIEW_CHANNEL,
                kind: PermissionOverwriteType::Role(guild_role),
            },
            PermissionOverwrite {
                allow: Permissions::VIEW_CHANNEL,
                deny: Permissions::SEND_TTS_MESSAGES,
                kind: PermissionOverwriteType::Member(self.component.user.id),
            },
        ];

        let parent_id = ChannelId::new(1212488323277525089);

        let channel_title = format!("ticket-{}", self.component.user.name);

        let channel_topic = format!("ticket-{}", self.component.user.id);

        let channel = CreateChannel::new(channel_title)
            .kind(ChannelType::Text)
            .topic(channel_topic)
            .category(parent_id)
            .permissions(permissions);

        guild.create_channel(&self.ctx.http, channel).await.unwrap()
    }

    pub fn create_channel_embed(&self) -> CreateEmbed {
        let member_avatar_url = self.component.user.avatar_url().unwrap();

        let embed_description = format!("- Olá <@{}>, seja bem-vindo ao seu atendimento!\n> Estamos aqui para ajudá-lo. Nossa equipe estará pronta para atendê-lo em breve. Agradecemos pela sua paciência.", self.component.user.id);

        let embed_footer =
            CreateEmbedFooter::new("Quantum Codes - Ticket").icon_url(member_avatar_url);

        let embed_timestamp = Timestamp::now();

        CreateEmbed::default()
            .title("<:staff:1213602173297098822> - Categoria suporte")
            .description(embed_description)
            .footer(embed_footer)
            .timestamp(embed_timestamp)
            .colour(Colour::LIGHT_GREY)
    }

    pub fn create_response_embed(&self, ticket_channel_id: ChannelId) -> CreateEmbed {
        let embed_description = format!(
            "- Olá <@{}>, seu novo ticket foi aberto.\n> Você pode encontrar ele aqui <#{}>.",
            self.component.user.id, ticket_channel_id
        );

        let embed_footer = CreateEmbedFooter::new("Quantum Codes - Ticket");

        let embed_timestamp = Timestamp::now();

        CreateEmbed::default()
            .description(embed_description)
            .footer(embed_footer)
            .timestamp(embed_timestamp)
            .colour(Colour::LIGHT_GREY)
    }

    pub fn create_channel_buttons(&self) -> CreateActionRow {
        let close_button = CreateButton::new("ticket-button-close")
            .style(ButtonStyle::Danger)
            .label("Fechar ticket");

        //TODO: add after
        // let add_member_button = CreateButton::new("ticket-button-add-member")
        //     .style(ButtonStyle::Primary)
        //     .label("Adicionar pessoa");

        CreateActionRow::Buttons(vec![close_button])
    }

    pub async fn send_ticket_embed(
        &self,
        channel: &GuildChannel,
        embed: CreateEmbed,
        action_row: CreateActionRow,
    ) {
        let response = CreateMessage::new()
            .add_embed(embed)
            .components(vec![action_row]);

        channel
            .send_message(&self.ctx.http, response)
            .await
            .unwrap();
    }

    pub async fn send_response_embed(&self, embed: CreateEmbed) {
        let response = EditInteractionResponse::new().embed(embed);

        self.component
            .edit_response(&self.ctx.http, response)
            .await
            .unwrap();
    }
}

pub async fn run(ctx: Context, component: ComponentInteraction) {
    component.defer_ephemeral(&ctx.http).await.unwrap();

    let ticket = Ticket::new(ctx, component);

    let channel_ticket = ticket.create_channel().await;

    let embed_ticket = ticket.create_channel_embed();

    let buttons_ticket = ticket.create_channel_buttons();

    ticket
        .send_ticket_embed(&channel_ticket, embed_ticket, buttons_ticket)
        .await;

    let embed_response = ticket.create_response_embed(channel_ticket.id);

    ticket.send_response_embed(embed_response).await;
}
