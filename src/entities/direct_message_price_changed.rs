use serde::{Deserialize, Serialize};

/// Describes a service message about a change in the price of direct messages sent to a channel chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#directmessagepricechanged)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectMessagePriceChanged {
    /// *True*, if direct messages are enabled for the channel chat; false otherwise
    pub are_direct_messages_enabled: bool,

    /// *Optional*. The new number of Telegram Stars that must be paid by users for each direct message sent to the channel. Does not apply to users who have been exempted by administrators. Defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_message_star_count: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
