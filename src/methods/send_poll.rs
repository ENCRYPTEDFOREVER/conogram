use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{
        input_poll_option::InputPollOption,
        message::Message,
        message_entity::MessageEntity,
        misc::{chat_id::ChatId, message_effects::MessageEffect, reply_markup::ReplyMarkup},
        poll::PollType,
        reply_parameters::ReplyParameters,
    },
    utils::deserialize_utils::is_false,
};

/// Use this method to send a native poll. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#sendpoll)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = Message)]
pub struct SendPollParams {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatId,

    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// Poll question, 1-300 characters
    pub question: String,

    /// Mode for parsing entities in the question. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Currently, only custom emoji entities are allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of *question\_parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub question_entities: Vec<MessageEntity>,

    /// A JSON-serialized list of 2-10 answer options
    pub options: Vec<InputPollOption>,

    /// *True*, if the poll needs to be anonymous, defaults to *True*
    #[serde(skip_serializing_if = "is_false")]
    pub is_anonymous: bool,

    /// Poll type, “quiz” or “regular”, defaults to “regular”
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<PollType>,

    /// *True*, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to *False*
    #[serde(skip_serializing_if = "is_false")]
    pub allows_multiple_answers: bool,

    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,

    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Mode for parsing entities in the explanation. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of *explanation\_parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub explanation_entities: Vec<MessageEntity>,

    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with *close\_date*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,

    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with *open\_period*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,

    /// Pass *True* if the poll needs to be immediately closed. This can be useful for poll preview.
    #[serde(skip_serializing_if = "is_false")]
    pub is_closed: bool,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "is_false")]
    pub disable_notification: bool,

    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "is_false")]
    pub protect_content: bool,

    /// Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    #[serde(skip_serializing_if = "is_false")]
    pub allow_paid_broadcast: bool,

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<MessageEffect>,

    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

// Divider: all content below this line will be preserved after code regen
