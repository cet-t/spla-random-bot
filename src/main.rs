mod commands;
mod macros;
mod splatoon;
mod toolkit;

use poise::FrameworkOptions;
use seren_ex::*;
use serenity::all::{ActivityData, GatewayIntents, OnlineStatus};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let intents = GatewayIntents::MESSAGE_CONTENT | GatewayIntents::DIRECT_MESSAGES;

    seren_ex::start(StartOptions {
        token_key: Some("TOKEN"),
        intents,
        framework: FrameworkConfig {
            options: FrameworkOptions {
                commands: crate::commands::commands(),
                ..Default::default()
            },
            setup: SetupOptions {
                presence: Some(PresenceOptions {
                    activity: Some(ActivityData::listening("packaging...")),
                    status: OnlineStatus::DoNotDisturb,
                }),
                registration: CommandRegistration::Global,
                log_ready: true,
            },
            common_commands: None,
        },
    })
    .await?;

    Ok(())
}
