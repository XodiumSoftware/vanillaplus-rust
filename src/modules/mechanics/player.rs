use crate::modules::module::Module;

use pumpkin_plugin_api::events::{
    EventHandler, EventPriority, PlayerJoinEventData, PlayerLeaveEventData,
};
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Handles player join and leave mechanics, including custom messages.
#[derive(Default)]
pub struct Player {
    /// Configuration for this module.
    config: Config,
}

impl Module for Player {
    fn enabled(&self) -> bool {
        self.config.enabled
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler(Player::default(), EventPriority::Highest, true)
            .expect("failed to register player event handler");
    }
}

impl EventHandler<PlayerJoinEventData> for Player {
    fn handle(&self, _server: Server, mut event: PlayerJoinEventData) -> PlayerJoinEventData {
        if self.enabled() {
            event.join_message = TextComponent::text(
                self.config
                    .join_msg
                    .replace("{player}", &event.player.name)
                    .as_str(),
            )
        }

        event
    }
}

impl EventHandler<PlayerLeaveEventData> for Player {
    fn handle(&self, _server: Server, mut event: PlayerLeaveEventData) -> PlayerLeaveEventData {
        if self.enabled() {
            event.leave_message = TextComponent::text(
                self.config
                    .leave_msg
                    .replace("{player}", &event.player.name)
                    .as_str(),
            );
        }

        event
    }
}

/// Configuration for the player mechanics module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Whether this module is active.
    pub enabled: bool,
    /// Message broadcast when a player joins. Use `{player}` as a placeholder for the player identifier.
    pub join_msg: String,
    /// Message broadcast when a player leaves. Use `{player}` as a placeholder for the player identifier.
    pub leave_msg: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: false,
            join_msg: "<green>➕<reset> <gradient:#FFE259:#FFA751>›</gradient> {player}!".into(),
            leave_msg: "<red>➖<reset> <gradient:#FFE259:#FFA751>›</gradient> {player}!".into(),
        }
    }
}
