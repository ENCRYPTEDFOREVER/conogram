use serde::{Deserialize, Serialize};

///The background is a freeform gradient that rotates after every message in the chat.
///
///API Reference: [link](https://core.telegram.org/bots/api/#backgroundfillfreeformgradient)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BackgroundFillFreeformGradient {
    ///A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
    pub colors: Vec<i64>,
}
// Divider: all content below this line will be preserved after code regen
