use serde::{Deserialize, Serialize};

use crate::entities::chat_share_theme_backdrop::ChatShareThemeBackdrop;

/// This object contains a list of backdrops, which can be used as the background of a theme of the screen for sharing a link to a user or a chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#chatsharethemebackdrops)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatShareThemeBackdrops {
    /// The list of backdrops
    pub backdrops: Vec<ChatShareThemeBackdrop>,
}

// Divider: all content below this line will be preserved after code regen
