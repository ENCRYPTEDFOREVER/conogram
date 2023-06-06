use serde::{Deserialize, Serialize};

///This object represents an animated emoji that displays a random value.
///API Reference: [link](https://core.telegram.org/bots/api/#dice)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Dice {
    ///Emoji on which the dice throw animation is based
    pub emoji: String,

    ///Value of the dice, 1-6 for “🎲”, “🎯” and “🎳” base emoji, 1-5 for “🏀” and “⚽” base emoji, 1-64 for “🎰” base emoji
    pub value: i64,
}
// Divider: all content below this line will be preserved after code regen
