use serde::{Deserialize, Serialize};

///This object represents the bot's short description.
///API Reference: [link](https://core.telegram.org/bots/api/#botshortdescription)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BotShortDescription {
    ///The bot's short description
    pub short_description: String,
}
// Divider: all content below this line will be preserved after code regen
