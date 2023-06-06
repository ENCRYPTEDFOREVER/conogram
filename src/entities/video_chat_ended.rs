use serde::{Deserialize, Serialize};

///This object represents a service message about a video chat ended in the chat.
///API Reference: [link](https://core.telegram.org/bots/api/#videochatended)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct VideoChatEnded {
    ///Video chat duration in seconds
    pub duration: i64,
}
// Divider: all content below this line will be preserved after code regen
