use serde::{Deserialize, Serialize};

///This object represents a message about a forwarded story in the chat. Currently holds no information.
///API Reference: [link](https://core.telegram.org/bots/api/#story)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Story {}
// Divider: all content below this line will be preserved after code regen
