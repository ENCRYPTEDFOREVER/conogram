use serde::{Deserialize, Serialize};

///Describes why a request was unsuccessful.
///
///API Reference: [link](https://core.telegram.org/bots/api/#responseparameters)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ResponseParameters {
    ///*Optional*. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    ///*Optional*. In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<i64>,
}
// Divider: all content below this line will be preserved after code regen
