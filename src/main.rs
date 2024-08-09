use poise::serenity_prelude as serenity;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

mod secrets;
mod ai;

/// Ask Gemini 1.5 Pro
#[poise::command(slash_command, prefix_command)]
async fn ask(
    ctx: Context<'_>,
    #[description = "Prompt"] prompt: String,
) -> Result<(), Error> {
    // let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = ai::ask(&prompt[..]).await;
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = secrets::BOT_TOKEN.to_string();
    let intents = serenity::GatewayIntents::non_privileged();

    // ai::ask("HELLO").await;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ask()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}