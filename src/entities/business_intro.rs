use serde::{Deserialize, Serialize};

use crate::entities::sticker::Sticker;

/// Contains information about the start page settings of a Telegram Business account.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#businessintro)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BusinessIntro {
    /// *Optional*. Title text of the business intro
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// *Optional*. Message text of the business intro
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// *Optional*. Sticker of the business intro
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
}

// Divider: all content below this line will be preserved after code regen
