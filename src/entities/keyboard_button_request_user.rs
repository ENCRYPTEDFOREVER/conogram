use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///This object defines the criteria used to request a suitable user. The identifier of the selected user will be shared with the bot when the corresponding button is pressed. [More about requesting users »](https://core.telegram.org/bots/features#chat-and-user-selection)
///API Reference: [link](https://core.telegram.org/bots/api/#keyboardbuttonrequestuser)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonRequestUser {
    ///Signed 32-bit identifier of the request, which will be received back in the [UserShared](https://core.telegram.org/bots/api/#usershared) object. Must be unique within the message
    pub request_id: i64,

    ///*Optional*. Pass *True* to request a bot, pass *False* to request a regular user. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "is_false", default)]
    pub user_is_bot: bool,

    ///*Optional*. Pass *True* to request a premium user, pass *False* to request a non-premium user. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "is_false", default)]
    pub user_is_premium: bool,
}
// Divider: all content below this line will be preserved after code regen
