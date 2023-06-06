use serde::{Deserialize, Serialize};

///This object represents the bot's description.
///API Reference: [link](https://core.telegram.org/bots/api/#botdescription)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BotDescription {
    ///The bot's description
    pub description: String,
}
// Divider: all content below this line will be preserved after code regen
