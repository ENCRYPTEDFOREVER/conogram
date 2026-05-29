use serde::{Deserialize, Serialize};

use crate::entities::{
    chat::Chat, message_entity::MessageEntity, poll_media::PollMedia, user::User,
};

/// This object contains information about one answer option in a poll.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#polloption)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PollOption {
    /// Unique identifier of the option, persistent on option addition and deletion
    pub persistent_id: String,

    /// Option text, 1-100 characters
    pub text: String,

    /// *Optional*. Special entities that appear in the option *text*. Currently, only custom emoji entities are allowed in poll option texts
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub text_entities: Vec<MessageEntity>,

    /// *Optional*. Media added to the poll option
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<PollMedia>,

    /// Number of users who voted for this option; may be 0 if unknown
    pub voter_count: i64,

    /// *Optional*. User who added the option; omitted if the option wasn't added by a user after poll creation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_by_user: Option<User>,

    /// *Optional*. Chat that added the option; omitted if the option wasn't added by a chat after poll creation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_by_chat: Option<Box<Chat>>,

    /// *Optional*. Point in time (Unix timestamp) when the option was added; omitted if the option existed in the original poll
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addition_date: Option<i64>,
}

// Divider: all content below this line will be preserved after code regen
