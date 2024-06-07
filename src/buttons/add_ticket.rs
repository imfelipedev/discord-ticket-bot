use serenity::all::{ComponentInteraction, Context};

pub async fn run(ctx: Context, component: ComponentInteraction) {
    component.defer_ephemeral(&ctx.http).await.unwrap();
}
