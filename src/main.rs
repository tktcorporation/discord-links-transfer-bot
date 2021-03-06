use std::env;

use serenity::{client::Client, framework::StandardFramework};

mod handler;
use handler::Handler;

mod commands;
use commands::GENERAL_GROUP;

mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix(
                &env::var("DISCORD_CMD_PREFIX")
                    .expect("Expected a command prefix in the environment"),
            )
        })
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    let _ = client
        .start()
        .await
        .map_err(|why| println!("Client ended: {:?}", why));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_env_exists() {
        env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        env::var("DISCORD_CMD_PREFIX").expect("Expected a prefix in the environment");
        assert!(true);
    }
}
