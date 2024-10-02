use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use serde::Serialize;

/// Represents a [Game](https://core.telegram.org/bots/api/#games).
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inlinequeryresultgame)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InlineQueryResultGame {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,

    /// Short name of the game
    pub game_short_name: String,

    /// *Optional*. [Inline keyboard](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

// Divider: all content below this line will be preserved after code regen
