use serde::{Deserialize, Serialize};

/// This object represents a service message about a video chat started in the chat. Currently holds no information.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#videochatstarted)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoChatStarted {}

// Divider: all content below this line will be preserved after code regen
