use serde::{Deserialize, Serialize};

/// The paid media isn't available before the payment.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#paidmediapreview)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaidMediaPreview {
    /// *Optional*. Media width as defined by the sender
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,

    /// *Optional*. Media height as defined by the sender
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,

    /// *Optional*. Duration of the media in seconds as defined by the sender
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
