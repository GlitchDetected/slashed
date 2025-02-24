use clap::{error::ErrorKind, CommandFactory, Parser};
use dialoguer::{Confirm, Input};
use poise::{
    serenity_prelude::{self as serenity}
};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// rm all cmds w/ no confirmation
    #[clap(short, long, action)]
    pub yes: bool,
}

pub fn safe_exit(code: i32) -> ! {
    use std::io::Write;
    let _ = std::io::stdout().lock().flush();
    let _ = std::io::stderr().lock().flush();
    std::process::exit(code)
}

/// none
#[poise::command(slash_command, prefix_command)]
async fn none(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let token: String = Input::new()
        .with_prompt("Enter your Discord bot token")
        .interact_text()?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![none()],
            ..Default::default()
        })
        .token(token.clone())
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |ctx, ready, _framework| {
            let cli = cli.clone();
            Box::pin(async move {
                if cli.yes
                    || Confirm::new()
                        .with_prompt(format!(
                            "Do you want to remove all of your commands for {}?",
                            ready.user.tag()
                        ))
                        .interact()?
                {
                    poise::builtins::register_globally::<Context<'_>, Error>(ctx, &[]).await?;
                    println!("Removed commands successfully!");
                }

                safe_exit(0);
            })
        });

    let result = framework.run().await;

    match result {
        Ok(_) => (),
        Err(err) => Cli::command().error(ErrorKind::InvalidValue, err).exit(),
    }

    Ok(())
}
