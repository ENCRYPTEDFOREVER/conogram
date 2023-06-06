use crate::entities::chat_administrator_rights::ChatAdministratorRights;
use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///This object defines the criteria used to request a suitable chat. The identifier of the selected chat will be shared with the bot when the corresponding button is pressed. [More about requesting chats »](https://core.telegram.org/bots/features#chat-and-user-selection)
///API Reference: [link](https://core.telegram.org/bots/api/#keyboardbuttonrequestchat)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestChat {
    ///Signed 32-bit identifier of the request, which will be received back in the [ChatShared](https://core.telegram.org/bots/api/#chatshared) object. Must be unique within the message
    pub request_id: i64,

    ///Pass *True* to request a channel chat, pass *False* to request a group or a supergroup chat.
    pub chat_is_channel: bool,

    ///*Optional*. Pass *True* to request a forum supergroup, pass *False* to request a non-forum chat. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "is_false", default)]
    pub chat_is_forum: bool,

    ///*Optional*. Pass *True* to request a supergroup or a channel with a username, pass *False* to request a chat without a username. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "is_false", default)]
    pub chat_has_username: bool,

    ///*Optional*. Pass *True* to request a chat owned by the user. Otherwise, no additional restrictions are applied.
    #[serde(skip_serializing_if = "is_false", default)]
    pub chat_is_created: bool,

    ///*Optional*. A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of *bot\_administrator\_rights*. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<ChatAdministratorRights>,

    ///*Optional*. A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of *user\_administrator\_rights*. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,

    ///*Optional*. Pass *True* to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
    #[serde(skip_serializing_if = "is_false", default)]
    pub bot_is_member: bool,
}
// Divider: all content below this line will be preserved after code regen
