use conogram_derives::Request;
use serde::Serialize;

use crate::{
    entities::{
        input_poll_media::InputPollMedia,
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

    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format `@username`. Polls can't be sent to channel direct messages chats.
    pub chat_id: ChatId,

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// Poll question, 1-300 characters
    pub question: String,

    /// Mode for parsing entities in the question. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Currently, only custom emoji entities are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in the poll question. It can be specified instead of *question\_parse\_mode*.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub question_entities: Vec<MessageEntity>,

    /// A JSON-serialized list of 1-12 answer options
    pub options: Vec<InputPollOption>,

    /// *True*, if the poll needs to be anonymous, defaults to *True*
    #[serde(skip_serializing_if = "is_false")]
    pub is_anonymous: bool,

    /// Poll type, “quiz” or “regular”, defaults to “regular”
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<PollType>,

    /// Pass *True*, if the poll allows multiple answers, defaults to *False*
    #[serde(skip_serializing_if = "is_false")]
    pub allows_multiple_answers: bool,

    /// Pass *True*, if the poll allows to change chosen answer options, defaults to *False* for quizzes and to *True* for regular polls
    #[serde(skip_serializing_if = "is_false")]
    pub allows_revoting: bool,

    /// Pass *True*, if the poll options must be shown in random order
    #[serde(skip_serializing_if = "is_false")]
    pub shuffle_options: bool,

    /// Pass *True*, if answer options can be added to the poll after creation; not supported for anonymous polls and quizzes
    #[serde(skip_serializing_if = "is_false")]
    pub allow_adding_options: bool,

    /// Pass *True*, if poll results must be shown only after the poll closes
    #[serde(skip_serializing_if = "is_false")]
    pub hide_results_until_closes: bool,

    /// Pass *True*, if voting is limited to users who have been members of the chat where the poll is being sent for more than 24 hours; for channel chats only
    #[serde(skip_serializing_if = "is_false")]
    pub members_only: bool,

    /// A JSON-serialized list of 0-12 two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes indicating the countries from which users can vote in the poll; for channel chats only. Use “FT” as a country code to allow users with anonymous numbers to vote. If omitted or empty, then users from any country can participate in the poll.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub country_codes: Vec<String>,

    /// A JSON-serialized list of monotonically increasing 0-based identifiers of the correct answer options, required for polls in quiz mode
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub correct_option_ids: Vec<i64>,

    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Mode for parsing entities in the explanation. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in the poll explanation. It can be specified instead of *explanation\_parse\_mode*.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub explanation_entities: Vec<MessageEntity>,

    /// Media added to the quiz explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_media: Option<InputPollMedia>,

    /// Amount of time in seconds the poll will be active after creation, 5-2628000. Can't be used together with *close\_date*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,

    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 2628000 seconds in the future. Can't be used together with *open\_period*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,

    /// Pass *True* if the poll needs to be immediately closed. This can be useful for poll preview.
    #[serde(skip_serializing_if = "is_false")]
    pub is_closed: bool,

    /// Description of the poll to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Mode for parsing entities in the poll description. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_parse_mode: Option<String>,

    /// A JSON-serialized list of special entities that appear in the poll description, which can be specified instead of *description\_parse\_mode*
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub description_entities: Vec<MessageEntity>,

    /// Media added to the poll description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<InputPollMedia>,

    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "is_false")]
    pub disable_notification: bool,

    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "is_false")]
    pub protect_content: bool,

    /// Pass *True* to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[serde(skip_serializing_if = "is_false")]
    pub allow_paid_broadcast: bool,

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<MessageEffect>,

    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

// Divider: all content below this line will be preserved after code regen
