use conogram_derives::Request;
use serde::Serialize;

use crate::entities::message_entity::MessageEntity;

/// Gifts a Telegram Premium subscription to the given user. Returns *True* on success.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#giftpremiumsubscription)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = bool)]
pub struct GiftPremiumSubscriptionParams {
    /// Unique identifier of the target user who will receive a Telegram Premium subscription
    pub user_id: i64,

    /// Number of months the Telegram Premium subscription will be active for the user; must be one of 3, 6, or 12
    pub month_count: i64,

    /// Number of Telegram Stars to pay for the Telegram Premium subscription; must be 1000 for 3 months, 1500 for 6 months, and 2500 for 12 months
    pub star_count: i64,

    /// Text that will be shown along with the service message about the subscription; 0-128 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\_emoji” are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of *text\_parse\_mode*. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\_emoji” are ignored.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text_entities: Vec<MessageEntity>,
}

// Divider: all content below this line will be preserved after code regen
