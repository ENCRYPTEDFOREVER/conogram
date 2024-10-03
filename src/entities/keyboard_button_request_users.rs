use serde::{Deserialize, Serialize};

use crate::utils::deserialize_utils::is_false;

/// This object defines the criteria used to request suitable users. Information about the selected users will be shared with the bot when the corresponding button is pressed. [More about requesting users »](https://core.telegram.org/bots/features#chat-and-user-selection)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#keyboardbuttonrequestusers)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestUsers {
    /// Signed 32-bit identifier of the request that will be received back in the [UsersShared](https://core.telegram.org/bots/api/#usersshared) object. Must be unique within the message
    pub request_id: i64,

    /// *Optional*. Pass *True* to request bots, pass *False* to request regular users. If not specified, no additional restrictions are applied.
    #[serde(default, skip_serializing_if = "is_false")]
    pub user_is_bot: bool,

    /// *Optional*. Pass *True* to request premium users, pass *False* to request non-premium users. If not specified, no additional restrictions are applied.
    #[serde(default, skip_serializing_if = "is_false")]
    pub user_is_premium: bool,

    /// *Optional*. The maximum number of users to be selected; 1-10. Defaults to 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<i64>,

    /// *Optional*. Pass *True* to request the users' first and last names
    #[serde(default, skip_serializing_if = "is_false")]
    pub request_name: bool,

    /// *Optional*. Pass *True* to request the users' usernames
    #[serde(default, skip_serializing_if = "is_false")]
    pub request_username: bool,

    /// *Optional*. Pass *True* to request the users' photos
    #[serde(default, skip_serializing_if = "is_false")]
    pub request_photo: bool,
}

// Divider: all content below this line will be preserved after code regen
