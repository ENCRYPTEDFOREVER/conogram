use serde::{Deserialize, Serialize};

///
///API Reference: [link](https://core.telegram.org/bots/api/#birthdate)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Birthdate {
    ///Day of the user's birth; 1-31
    pub day: i64,

    ///Month of the user's birth; 1-12
    pub month: i64,

    ///*Optional*. Year of the user's birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i64>,
}
// Divider: all content below this line will be preserved after code regen
