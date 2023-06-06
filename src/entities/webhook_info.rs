use serde::{Deserialize, Serialize};

///Describes the current status of a webhook.
///API Reference: [link](https://core.telegram.org/bots/api/#webhookinfo)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebhookInfo {
    ///Webhook URL, may be empty if webhook is not set up
    pub url: String,

    ///*True*, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,

    ///Number of updates awaiting delivery
    pub pending_update_count: i64,

    ///*Optional*. Currently used webhook IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    ///*Optional*. Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i64>,

    ///*Optional*. Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,

    ///*Optional*. Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<i64>,

    ///*Optional*. The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,

    ///*Optional*. A list of update types the bot is subscribed to. Defaults to all update types except *chat\_member*
    #[serde(default)]
    pub allowed_updates: Vec<String>,
}
// Divider: all content below this line will be preserved after code regen
