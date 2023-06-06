use serde::{Deserialize, Serialize};

///This object represents a service message about a change in auto-delete timer settings.
///API Reference: [link](https://core.telegram.org/bots/api/#messageautodeletetimerchanged)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    ///New auto-delete time for messages in the chat; in seconds
    pub message_auto_delete_time: i64,
}
// Divider: all content below this line will be preserved after code regen
