use crate::config::ConfigManager;
use crate::module::Module;
use pumpkin_plugin_api::events::{EventData, EventHandler, EventPriority, PlayerJoinEvent};
use pumpkin_plugin_api::text::TextComponent;
use pumpkin_plugin_api::{Context, Server};
use serde::{Deserialize, Serialize};

/// Handles tab-list mechanics, including custom messages.
#[derive(Default)]
pub struct Tablist;

impl Module for Tablist {
    fn enabled(&self) -> bool {
        ConfigManager::get()
            .map(|cm| cm.get_config::<Config>().enabled)
            .unwrap_or(true)
    }

    fn events(&self, context: &Context) {
        context
            .register_event_handler::<PlayerJoinEvent, _>(Tablist, EventPriority::Normal, true)
            .expect("failed to register tablist event handler");
    }
}

impl EventHandler<PlayerJoinEvent> for Tablist {
    fn handle(
        &self,
        _server: Server,
        event: EventData<PlayerJoinEvent>,
    ) -> EventData<PlayerJoinEvent> {
        let config: Config = ConfigManager::get()
            .map(|cm| cm.get_config())
            .unwrap_or_default();

        if !self.enabled() {
            return event;
        }

        let header = TextComponent::text(&config.header);
        let footer = TextComponent::text(&config.footer);
        event.player.set_tab_list_header_footer(header, footer);

        event
    }
}

/// Configuration for the tablist mechanics module.
pub type TablistConfig = Config;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Whether this module is active.
    pub enabled: bool,
    /// Header text displayed at the top of the tab list. Supports Minecraft formatting codes. Leave empty to disable.
    pub header: String,
    /// Footer text displayed at the bottom of the tab list. Supports Minecraft formatting codes. Leave empty to disable.
    pub footer: String,
}
