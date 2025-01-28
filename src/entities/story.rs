use serde::{Deserialize, Serialize};

use crate::entities::chat::Chat;

/// This object represents a story.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#story)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Story {
    /// Chat that posted the story
    pub chat: Box<Chat>,

    /// Unique identifier for the story in the chat
    pub id: i64,
}

// Divider: all content below this line will be preserved after code regen
