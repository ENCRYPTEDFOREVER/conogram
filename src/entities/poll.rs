use serde::{Deserialize, Serialize};

use crate::entities::{
    message_entity::MessageEntity, poll_media::PollMedia, poll_option::PollOption,
};

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

    /// *True*, if the poll allows to change the chosen answer options
    pub allows_revoting: bool,

    /// *True* if voting is limited to users who have been members of the chat where the poll was originally sent for more than 24 hours
    pub members_only: bool,

    /// *Optional*. A list of two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes indicating the countries from which users can vote in the poll. The country code “FT” is used for users with anonymous numbers. If omitted, then users from any country can participate in the poll.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub country_codes: Vec<String>,

    /// *Optional*. Array of 0-based identifiers of the correct answer options. Available only for polls in quiz mode which are closed or were sent (not forwarded) by the bot or to the private chat with the bot.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub correct_option_ids: Vec<i64>,

    /// *Optional*. Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// *Optional*. Special entities like usernames, URLs, bot commands, etc. that appear in the *explanation*
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub explanation_entities: Vec<MessageEntity>,

    /// *Optional*. Media added to the quiz explanation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation_media: Option<PollMedia>,

    /// *Optional*. Amount of time in seconds the poll will be active after creation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,

    /// *Optional*. Point in time (Unix timestamp) when the poll will be automatically closed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,

    /// *Optional*. Description of the poll; for polls inside the [Message](https://core.telegram.org/bots/api/#message) object only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// *Optional*. Special entities like usernames, URLs, bot commands, etc. that appear in the description
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description_entities: Vec<MessageEntity>,

    /// *Optional*. Media added to the poll description; for polls inside the [Message](https://core.telegram.org/bots/api/#message) object only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<PollMedia>,
}

/// Poll type, currently can be “regular” or “quiz”
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
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
