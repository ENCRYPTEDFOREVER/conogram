use serde::{Deserialize, Serialize};

///Describes data sent from a [Web App](https://core.telegram.org/bots/webapps) to the bot.
///API Reference: [link](https://core.telegram.org/bots/api/#webappdata)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WebAppData {
    ///The data. Be aware that a bad client can send arbitrary data in this field.
    pub data: String,

    ///Text of the *web\_app* keyboard button from which the Web App was opened. Be aware that a bad client can send arbitrary data in this field.
    pub button_text: String,
}
// Divider: all content below this line will be preserved after code regen
