use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
///API Reference: [link](https://core.telegram.org/bots/api/#switchinlinequerychosenchat)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    ///*Optional*. The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    ///*Optional*. True, if private chats with users can be chosen
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_user_chats: bool,

    ///*Optional*. True, if private chats with bots can be chosen
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_bot_chats: bool,

    ///*Optional*. True, if group and supergroup chats can be chosen
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_group_chats: bool,

    ///*Optional*. True, if channel chats can be chosen
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_channel_chats: bool,
}
// Divider: all content below this line will be preserved after code regen
