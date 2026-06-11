use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// Formatted date and time.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextdatetime)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextDateTime {
    /// The text
    pub text: Box<RichText>,

    /// The Unix time associated with the entity
    pub unix_time: i64,

    /// The string that defines the formatting of the date and time. See [date-time entity formatting](https://core.telegram.org/bots/api/#date-time-entity-formatting) for more details.
    pub date_time_format: String,
}

// Divider: all content below this line will be preserved after code regen
