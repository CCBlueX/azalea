//! A simple bot that repeats chat messages sent by other players.

use std::time::Duration;

use azalea::prelude::*;
use azalea_protocol::packets::game::{clientbound_system_chat_packet::ClientboundSystemChatPacket, ClientboundGamePacket};
use tracing::info;

#[tokio::main]
async fn main() {
    let account = Account::offline("ubefubewi46");
    // or let account = Account::microsoft("email").await.unwrap();
    
    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "ccc.blocksmc.com")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
pub struct State {}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    if let Event::Login = event {
        bot.chat("/register");
    }

    if let Event::Chat(m) = event {
        if let (None, content) = m.split_sender_and_content() {
            if content.starts_with("/register") {
                info!("{} asked to register, executing in 12 seconds...", bot.username());
                tokio::time::sleep(Duration::from_secs(12)).await;
                bot.chat(&*format!("/register sample{} sample{}", bot.username(), bot.username()));
            }

            if content.starts_with("/login") {
                info!("{} asked to login, executing in 12 seconds...", bot.username());
                tokio::time::sleep(Duration::from_secs(12)).await;
                bot.chat(&*format!("/login sample{}", bot.username()));
            }
        };
    }

    Ok(())
}
