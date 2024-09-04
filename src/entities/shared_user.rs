use crate::entities::photo_size::PhotoSize;
use serde::{Deserialize, Serialize};

///This object contains information about a user that was shared with the bot using a [KeyboardButtonRequestUsers](https://core.telegram.org/bots/api/#keyboardbuttonrequestusers) button.
///
///API Reference: [link](https://core.telegram.org/bots/api/#shareduser)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SharedUser {
    ///Identifier of the shared user. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the user and could be unable to use this identifier, unless the user is already known to the bot by some other means.
    pub user_id: i64,

    ///*Optional*. First name of the user, if the name was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    ///*Optional*. Last name of the user, if the name was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    ///*Optional*. Username of the user, if the username was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    ///*Optional*. Available sizes of the chat photo, if the photo was requested by the bot
    #[serde(default)]
    pub photo: Vec<PhotoSize>,
}
// Divider: all content below this line will be preserved after code regen
