use serde::{Deserialize, Serialize};

use crate::entities::rich_text::RichText;

/// A bot command.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richtextbotcommand)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RichTextBotCommand {
    /// The text
    pub text: Box<RichText>,

    /// The bot command
    pub bot_command: String,
}

// Divider: all content below this line will be preserved after code regen
