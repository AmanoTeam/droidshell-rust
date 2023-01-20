use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run(Environment::new("TELEGRAM_BOT_TOKEN")).await;
}

#[teloxide(command("start"))]
async fn start(cx: CommandCx<Message>) -> TeloxideResult<()> {
    cx.answer("Hello, world!").send().await?;
    Ok(())
}