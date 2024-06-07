use super::events;
use serenity::all::Interaction;
use serenity::all::Ready;
use serenity::async_trait;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, interaction: Ready) {
        events::ready::run(ctx, interaction).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        events::interaction_create::run(ctx, interaction).await;
    }
}
