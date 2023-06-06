use serde::{Deserialize, Serialize};

///This object represents a service message about a video chat scheduled in the chat.
///API Reference: [link](https://core.telegram.org/bots/api/#videochatscheduled)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct VideoChatScheduled {
    ///Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    pub start_date: i64,
}
// Divider: all content below this line will be preserved after code regen
