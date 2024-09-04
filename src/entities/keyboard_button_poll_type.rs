use serde::{Deserialize, Serialize};

///This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
///
///API Reference: [link](https://core.telegram.org/bots/api/#keyboardbuttonpolltype)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonPollType {
    ///*Optional*. If *quiz* is passed, the user will be allowed to create only polls in the quiz mode. If *regular* is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
// Divider: all content below this line will be preserved after code regen
