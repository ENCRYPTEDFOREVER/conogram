use serde::{Deserialize, Serialize};

/// This object represents an animated emoji that displays a random value.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#dice)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,

    /// Value of the dice, 1-6 for “🎲”, “🎯” and “🎳” base emoji, 1-5 for “🏀” and “⚽” base emoji, 1-64 for “🎰” base emoji
    pub value: i64,
}

// Divider: all content below this line will be preserved after code regen
impl Dice {
    ///  If dice animation suggests a win
    #[must_use]
    pub fn is_winning(&self) -> bool {
        match self.emoji.as_str() {
            "🎰" => matches!(self.value, 1 | 22 | 43 | 64),
            "🎳" | "🎲" | "🎯" => self.value == 6,
            "⚽" => self.value >= 3,
            "🏀" => self.value >= 4,
            _ => false,
        }
    }

    ///  If dice value is winning value
    #[must_use]
    pub fn is_winning_canon(&self) -> bool {
        match self.emoji.as_str() {
            "🎰" => self.value == 64,
            "🎳" | "🎲" | "🎯" => self.value == 6,
            "⚽" | "🏀" => self.value == 5,
            _ => false,
        }
    }
}
