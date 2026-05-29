use conogram_derives::Request;
use serde::Serialize;

use crate::entities::bot_access_settings::BotAccessSettings;

/// Use this method to get the access settings of a managed bot. Returns a [BotAccessSettings](https://core.telegram.org/bots/api/#botaccesssettings) object on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getmanagedbotaccesssettings)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = BotAccessSettings)]
pub struct GetManagedBotAccessSettingsParams {
    /// User identifier of the managed bot whose access settings will be returned
    pub user_id: i64,
}

// Divider: all content below this line will be preserved after code regen
