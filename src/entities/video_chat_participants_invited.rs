use serde::{Deserialize, Serialize};

use crate::entities::user::User;

/// This object represents a service message about new members invited to a video chat.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#videochatparticipantsinvited)
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat
    pub users: Vec<User>,
}

// Divider: all content below this line will be preserved after code regen
