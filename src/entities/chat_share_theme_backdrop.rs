use serde::{Deserialize, Serialize};

use crate::entities::unique_gift_backdrop_colors::UniqueGiftBackdropColors;

/// This object describes a backdrop, which can be used as the background of a theme of the screen for sharing a link to a user or a chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatsharethemebackdrop)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatShareThemeBackdrop {
    /// Unique identifier of the backdrop
    pub id: i64,

    /// Name of the backdrop
    pub name: String,

    /// Colors of the backdrop
    pub colors: UniqueGiftBackdropColors,
}

// Divider: all content below this line will be preserved after code regen
