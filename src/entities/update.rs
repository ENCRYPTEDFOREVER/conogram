use crate::entities::callback_query::CallbackQuery;
use crate::entities::chat_boost_removed::ChatBoostRemoved;
use crate::entities::chat_boost_updated::ChatBoostUpdated;
use crate::entities::chat_join_request::ChatJoinRequest;
use crate::entities::chat_member_updated::ChatMemberUpdated;
use crate::entities::chosen_inline_result::ChosenInlineResult;
use crate::entities::inline_query::InlineQuery;
use crate::entities::message::Message;
use crate::entities::message_reaction_count_updated::MessageReactionCountUpdated;
use crate::entities::message_reaction_updated::MessageReactionUpdated;
use crate::entities::poll::Poll;
use crate::entities::poll_answer::PollAnswer;
use crate::entities::pre_checkout_query::PreCheckoutQuery;
use crate::entities::shipping_query::ShippingQuery;
use serde::{Deserialize, Serialize};

///This [object](https://core.telegram.org/bots/api/#available-types) represents an incoming update.  
///At most **one** of the optional parameters can be present in any given update.
///API Reference: [link](https://core.telegram.org/bots/api/#update)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Update {
    ///The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using [webhooks](https://core.telegram.org/bots/api/#setwebhook), since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,

    ///*Optional*. New incoming message of any kind - text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,

    ///*Optional*. New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Box<Message>>,

    ///*Optional*. New incoming channel post of any kind - text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Box<Message>>,

    ///*Optional*. New version of a channel post that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Box<Message>>,

    ///*Optional*. A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify `"message_reaction"` in the list of *allowed\_updates* to receive these updates. The update isn't received for reactions set by bots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction: Option<MessageReactionUpdated>,

    ///*Optional*. Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify `"message_reaction_count"` in the list of *allowed\_updates* to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction_count: Option<MessageReactionCountUpdated>,

    ///*Optional*. New incoming [inline](https://core.telegram.org/bots/api/#inline-mode) query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,

    ///*Optional*. The result of an [inline](https://core.telegram.org/bots/api/#inline-mode) query that was chosen by a user and sent to their chat partner. Please see our documentation on the [feedback collecting](https://core.telegram.org/bots/inline#collecting-feedback) for details on how to enable these updates for your bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,

    ///*Optional*. New incoming callback query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,

    ///*Optional*. New incoming shipping query. Only for invoices with flexible price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,

    ///*Optional*. New incoming pre-checkout query. Contains full information about checkout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,

    ///*Optional*. New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    ///*Optional*. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,

    ///*Optional*. The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,

    ///*Optional*. A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify `"chat_member"` in the list of *allowed\_updates* to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,

    ///*Optional*. A request to join the chat has been sent. The bot must have the *can\_invite\_users* administrator right in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,

    ///*Optional*. A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_boost: Option<ChatBoostUpdated>,

    ///*Optional*. A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_chat_boost: Option<ChatBoostRemoved>,
}
// Divider: all content below this line will be preserved after code regen

pub enum AllowedUpdates {
    Message,
    EditedMessage,

    ChannelPost,
    EditedChannelPost,

    MessageReaction,
    MessageReactionCount,

    InlineQuery,
    ChosenInlineResult,

    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,

    Poll,
    PollAnswer,

    MyChatMember,
    ChatMember,

    ChatJoinRequest,

    ChatBoost,
    RemovedChatBoost,
}

impl AllowedUpdates {
    /// All existing updates
    pub fn all() -> Vec<Self> {
        vec![
            Self::Message,
            Self::EditedMessage,
            Self::MessageReaction,
            Self::MessageReactionCount,
            Self::ChannelPost,
            Self::EditedChannelPost,
            Self::InlineQuery,
            Self::ChosenInlineResult,
            Self::CallbackQuery,
            Self::ShippingQuery,
            Self::PreCheckoutQuery,
            Self::Poll,
            Self::PollAnswer,
            Self::MyChatMember,
            Self::ChatMember,
            Self::ChatJoinRequest,
            Self::ChatBoost,
            Self::RemovedChatBoost,
        ]
    }
}

impl ToString for AllowedUpdates {
    fn to_string(&self) -> String {
        match self {
            AllowedUpdates::Message => "message".into(),
            AllowedUpdates::EditedMessage => "edited_message".into(),
            AllowedUpdates::MessageReaction => "message_reaction".into(),
            AllowedUpdates::MessageReactionCount => "message_reaction_count".into(),
            AllowedUpdates::ChannelPost => "channel_post".into(),
            AllowedUpdates::EditedChannelPost => "edited_channel_post".into(),
            AllowedUpdates::InlineQuery => "inline_query".into(),
            AllowedUpdates::ChosenInlineResult => "chosen_inline_result".into(),
            AllowedUpdates::CallbackQuery => "callback_query".into(),
            AllowedUpdates::ShippingQuery => "shipping_query".into(),
            AllowedUpdates::PreCheckoutQuery => "pre_checkout_query".into(),
            AllowedUpdates::Poll => "poll".into(),
            AllowedUpdates::PollAnswer => "poll_answer".into(),
            AllowedUpdates::MyChatMember => "my_chat_member".into(),
            AllowedUpdates::ChatMember => "chat_member".into(),
            AllowedUpdates::ChatJoinRequest => "chat_join_request".into(),

            AllowedUpdates::ChatBoost => "chat_boost".into(),
            AllowedUpdates::RemovedChatBoost => "removed_chat_boost".into(),
        }
    }
}
