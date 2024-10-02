use crate::entities::message_entity::MessageEntity;
use crate::entities::poll_option::PollOption;
use serde::{Deserialize, Serialize};

/// This object contains information about a poll.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#poll)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,

    /// Poll question, 1-300 characters
    pub question: String,

    /// *Optional*. Special entities that appear in the *question*. Currently, only custom emoji entities are allowed in poll questions
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub question_entities: Vec<MessageEntity>,

    /// List of poll options
    pub options: Vec<PollOption>,

    /// Total number of users that voted in the poll
    pub total_voter_count: i64,

    /// *True*, if the poll is closed
    pub is_closed: bool,

    /// *True*, if the poll is anonymous
    pub is_anonymous: bool,

    /// Poll type, currently can be “regular” or “quiz”
    #[serde(rename = "type")]
    pub type_: PollType,

    /// *True*, if the poll allows multiple answers
    pub allows_multiple_answers: bool,

    /// *Optional*. 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,

    /// *Optional*. Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// *Optional*. Special entities like usernames, URLs, bot commands, etc. that appear in the *explanation*
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub explanation_entities: Vec<MessageEntity>,

    /// *Optional*. Amount of time in seconds the poll will be active after creation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,

    /// *Optional*. Point in time (Unix timestamp) when the poll will be automatically closed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
}

/// Poll type, currently can be “regular” or “quiz”
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum PollType {
    /// `regular`
    #[default]
    #[serde(rename = "regular")]
    Regular,

    /// `quiz`
    #[serde(rename = "quiz")]
    Quiz,
}

// Divider: all content below this line will be preserved after code regen
