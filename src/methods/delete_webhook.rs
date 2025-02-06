use conogram_derives::Request;
use serde::Serialize;

use crate::utils::deserialize_utils::is_false;

/// Use this method to remove webhook integration if you decide to switch back to [getUpdates](https://core.telegram.org/bots/api/#getupdates). Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#deletewebhook)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct DeleteWebhookParams {
    /// Pass *True* to drop all pending updates
    #[serde(skip_serializing_if = "is_false")]
    pub drop_pending_updates: bool,
}

// Divider: all content below this line will be preserved after code regen
