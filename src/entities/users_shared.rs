use serde::{Deserialize, Serialize};

///This object contains information about the users whose identifiers were shared with the bot using a [KeyboardButtonRequestUsers](https://core.telegram.org/bots/api/#keyboardbuttonrequestusers) button.
///API Reference: [link](https://core.telegram.org/bots/api/#usersshared)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersShared {
    ///Identifier of the request
    pub request_id: i64,

    ///Identifiers of the shared users. These numbers may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting them. But they have at most 52 significant bits, so 64-bit integers or double-precision float types are safe for storing these identifiers. The bot may not have access to the users and could be unable to use these identifiers, unless the users are already known to the bot by some other means.
    pub user_ids: Vec<i64>,
}
// Divider: all content below this line will be preserved after code regen
