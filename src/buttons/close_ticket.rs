use serenity::all::{
    ChannelId, Colour, ComponentInteraction, Context, CreateAttachment, CreateEmbed,
    CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, EditInteractionResponse, Timestamp,
    UserId,
};
use serenity::futures::StreamExt;
use std::io::Write;

struct Close {
    ctx: Context,
    component: ComponentInteraction,
}

impl Close {
    pub fn new(ctx: Context, component: ComponentInteraction) -> Close {
        Close { ctx, component }
    }

    pub async fn generate_transcript(&self) -> CreateAttachment {
        let mut buffer: Vec<u8> = Vec::new();

        let mut messages = self
            .component
            .channel_id
            .messages_iter(&self.ctx.http)
            .boxed();

        while let Some(message_result) = messages.next().await {
            match message_result {
                Ok(message) => {
                    let message = format!(
                        "[{}] - {} said: \"{}\".\n",
                        message.timestamp, message.author.name, message.content
                    );

                    buffer
                        .write_all(message.as_bytes())
                        .expect("Failed to write to buffer");
                }
                Err(error) => eprintln!("Uh oh! Error: {}", error),
            }
        }

        CreateAttachment::bytes(buffer, "transcript.txt")
    }

    pub fn create_log_embed(&self, owner: &String) -> CreateEmbed {
        let bot_avatar_url = self.ctx.cache.current_user().avatar_url().unwrap();

        let author = CreateEmbedAuthor::new("Logs do atendimento").icon_url(&bot_avatar_url);

        let description = format!("- E aí <@{}>, aqui está um resumo do seu atendimento.\n> Seu atendimento foi encerrado pelo usuário <@{}> Se surgirem mais dúvidas, sinta-se à vontade para entrar em contato novamente. A equipe Quantum Codes está à disposição!", owner, self.component.user.id);

        let footer = CreateEmbedFooter::new("Quantum Codes - Ticket").icon_url(bot_avatar_url);

        let timestamp = Timestamp::now();

        CreateEmbed::default()
            .author(author)
            .description(description)
            .footer(footer)
            .timestamp(timestamp)
            .colour(Colour::LIGHT_GREY)
    }

    pub fn get_ticket_owner(&self) -> String {
        let component = self.component.clone();

        let partial_channel = component.channel.unwrap();

        let channel = self.ctx.cache.channel(partial_channel.id).unwrap();

        let topic = channel.topic.as_ref().unwrap();

        let collection: Vec<&str> = topic.as_str().split("-").collect();

        collection[1].to_string()
    }

    pub async fn send_log(&self, embed: CreateEmbed, attachment: CreateAttachment) {
        let guild = self.component.guild_id.unwrap();

        let guild_channels = guild.channels(&self.ctx.http).await.unwrap();

        let channel_id = ChannelId::new(1212488323483041798);

        let channel = guild_channels.get(&channel_id).unwrap();

        let message = CreateMessage::new().embed(embed).add_file(attachment);

        channel.send_message(&self.ctx.http, message).await.unwrap();
    }

    pub async fn send_log_private(
        &self,
        id: String,
        embed: CreateEmbed,
        attachment: CreateAttachment,
    ) {
        let user_id = id.parse().unwrap();
        let user = UserId::new(user_id).to_user(&self.ctx).await.unwrap();
        let message = CreateMessage::new().embed(embed).add_file(attachment);
        user.dm(&self.ctx.http, message).await.unwrap();
    }

    pub async fn send_response_and_delete_channel(&self) {
        let response = EditInteractionResponse::new().content("Fechando ticket em 3, 2, 1");

        self.component
            .edit_response(&self.ctx.http, response)
            .await
            .unwrap();

        self.component
            .channel_id
            .delete(&self.ctx.http)
            .await
            .unwrap();
    }
}

pub async fn run(ctx: Context, component: ComponentInteraction) {
    component.defer_ephemeral(&ctx.http).await.unwrap();

    let close = Close::new(ctx, component);

    let owner = close.get_ticket_owner();

    let embed_log = close.create_log_embed(&owner);

    let transcript = close.generate_transcript().await;

    close
        .send_log_private(owner, embed_log.clone(), transcript.clone())
        .await;

    close.send_log(embed_log, transcript).await;

    close.send_response_and_delete_channel().await;
}
