use serde::{Deserialize, Serialize};

use crate::entities::shared_user::SharedUser;

/// This object contains information about the users whose identifiers were shared with the bot using a [KeyboardButtonRequestUsers](https://core.telegram.org/bots/api/#keyboardbuttonrequestusers) button.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#usersshared)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsersShared {
    /// Identifier of the request
    pub request_id: i64,

    /// Information about users shared with the bot.
    pub users: Vec<SharedUser>,
}

// Divider: all content below this line will be preserved after code regen
