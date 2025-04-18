use serde::{Deserialize, Serialize};

use crate::{
    entities::{
        keyboard_button_poll_type::KeyboardButtonPollType,
        keyboard_button_request_chat::KeyboardButtonRequestChat,
        keyboard_button_request_users::KeyboardButtonRequestUsers, web_app_info::WebAppInfo,
    },
    utils::deserialize_utils::is_false,
};

/// This object represents one button of the reply keyboard. At most one of the optional fields must be used to specify type of the button. For simple text buttons, *String* can be used instead of this object to specify the button text.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#keyboardbutton)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed
    pub text: String,

    /// *Optional.* If specified, pressing the button will open a list of suitable users. Identifiers of selected users will be sent to the bot in a “users\_shared” service message. Available in private chats only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_users: Option<KeyboardButtonRequestUsers>,

    /// *Optional.* If specified, pressing the button will open a list of suitable chats. Tapping on a chat will send its identifier to the bot in a “chat\_shared” service message. Available in private chats only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,

    /// *Optional*. If *True*, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    #[serde(default, skip_serializing_if = "is_false")]
    pub request_contact: bool,

    /// *Optional*. If *True*, the user's current location will be sent when the button is pressed. Available in private chats only.
    #[serde(default, skip_serializing_if = "is_false")]
    pub request_location: bool,

    /// *Optional*. If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,

    /// *Optional*. If specified, the described [Web App](https://core.telegram.org/bots/webapps) will be launched when the button is pressed. The Web App will be able to send a “web\_app\_data” service message. Available in private chats only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}

// Divider: all content below this line will be preserved after code regen
