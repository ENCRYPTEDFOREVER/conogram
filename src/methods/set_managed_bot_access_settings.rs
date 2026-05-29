use conogram_derives::Request;
use serde::Serialize;

/// Use this method to change the access settings of a managed bot. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#setmanagedbotaccesssettings)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct SetManagedBotAccessSettingsParams {
    /// User identifier of the managed bot whose access settings will be changed
    pub user_id: i64,

    /// Pass *True*, if only selected users can access the bot. The bot's owner can always access it.
    pub is_access_restricted: bool,

    /// A JSON-serialized list of up to 10 identifiers of users who will have access to the bot in addition to its owner. Ignored if *is\_access\_restricted* is false.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub added_user_ids: Vec<i64>,
}

// Divider: all content below this line will be preserved after code regen
