use serde::{Deserialize, Serialize};

use crate::entities::{
    business_connection::BusinessConnection, business_messages_deleted::BusinessMessagesDeleted,
    callback_query::CallbackQuery, chat_boost_removed::ChatBoostRemoved,
    chat_boost_updated::ChatBoostUpdated, chat_join_request::ChatJoinRequest,
    chat_member_updated::ChatMemberUpdated, chosen_inline_result::ChosenInlineResult,
    inline_query::InlineQuery, message::Message,
    message_reaction_count_updated::MessageReactionCountUpdated,
    message_reaction_updated::MessageReactionUpdated, paid_media_purchased::PaidMediaPurchased,
    poll::Poll, poll_answer::PollAnswer, pre_checkout_query::PreCheckoutQuery,
    shipping_query::ShippingQuery,
};

/// This [object](https://core.telegram.org/bots/api/#available-types) represents an incoming update.  
/// At most **one** of the optional parameters can be present in any given update.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#update)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using [webhooks](https://core.telegram.org/bots/api/#setwebhook), since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,

    /// *Optional*. New incoming message of any kind - text, photo, sticker, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,

    /// *Optional*. New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Box<Message>>,

    /// *Optional*. New incoming channel post of any kind - text, photo, sticker, etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Box<Message>>,

    /// *Optional*. New version of a channel post that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Box<Message>>,

    /// *Optional*. The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_connection: Option<BusinessConnection>,

    /// *Optional*. New message from a connected business account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_message: Option<Box<Message>>,

    /// *Optional*. New version of a message from a connected business account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_business_message: Option<Box<Message>>,

    /// *Optional*. Messages were deleted from a connected business account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_business_messages: Option<BusinessMessagesDeleted>,

    /// *Optional*. A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify `"message_reaction"` in the list of *allowed\_updates* to receive these updates. The update isn't received for reactions set by bots.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_reaction: Option<MessageReactionUpdated>,

    /// *Optional*. Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify `"message_reaction_count"` in the list of *allowed\_updates* to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_reaction_count: Option<MessageReactionCountUpdated>,

    /// *Optional*. New incoming [inline](https://core.telegram.org/bots/api/#inline-mode) query
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,

    /// *Optional*. The result of an [inline](https://core.telegram.org/bots/api/#inline-mode) query that was chosen by a user and sent to their chat partner. Please see our documentation on the [feedback collecting](https://core.telegram.org/bots/inline#collecting-feedback) for details on how to enable these updates for your bot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,

    /// *Optional*. New incoming callback query
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,

    /// *Optional*. New incoming shipping query. Only for invoices with flexible price
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,

    /// *Optional*. New incoming pre-checkout query. Contains full information about checkout
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,

    /// *Optional*. A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchased_paid_media: Option<PaidMediaPurchased>,

    /// *Optional*. New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    /// *Optional*. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,

    /// *Optional*. The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,

    /// *Optional*. A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify `"chat_member"` in the list of *allowed\_updates* to receive these updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,

    /// *Optional*. A request to join the chat has been sent. The bot must have the *can\_invite\_users* administrator right in the chat to receive these updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,

    /// *Optional*. A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_boost: Option<ChatBoostUpdated>,

    /// *Optional*. A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed_chat_boost: Option<ChatBoostRemoved>,
}

// Divider: all content below this line will be preserved after code regen
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum AllowedUpdates {
    Message,
    EditedMessage,

    ChannelPost,
    EditedChannelPost,

    BusinessConnection,
    BusinessMessage,
    EditedBusinessMessage,
    DeletedBusinessMessages,

    MessageReaction,
    MessageReactionCount,

    InlineQuery,
    ChosenInlineResult,

    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,

    PurchasedPaidMedia,

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
    pub const fn all() -> [Self; 23] {
        [
            Self::Message,
            Self::EditedMessage,
            Self::MessageReaction,
            Self::MessageReactionCount,
            Self::ChannelPost,
            Self::EditedChannelPost,
            Self::BusinessConnection,
            Self::BusinessMessage,
            Self::EditedBusinessMessage,
            Self::DeletedBusinessMessages,
            Self::InlineQuery,
            Self::ChosenInlineResult,
            Self::CallbackQuery,
            Self::ShippingQuery,
            Self::PreCheckoutQuery,
            Self::PurchasedPaidMedia,
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

impl Display for AllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Message => "message",
            Self::EditedMessage => "edited_message",
            Self::MessageReaction => "message_reaction",
            Self::MessageReactionCount => "message_reaction_count",
            Self::ChannelPost => "channel_post",
            Self::EditedChannelPost => "edited_channel_post",
            Self::BusinessConnection => "business_connection",
            Self::BusinessMessage => "business_message",
            Self::EditedBusinessMessage => "edited_business_message",
            Self::DeletedBusinessMessages => "deleted_business_messages",
            Self::InlineQuery => "inline_query",
            Self::ChosenInlineResult => "chosen_inline_result",
            Self::CallbackQuery => "callback_query",
            Self::ShippingQuery => "shipping_query",
            Self::PreCheckoutQuery => "pre_checkout_query",
            Self::PurchasedPaidMedia => "purchased_paid_media",
            Self::Poll => "poll",
            Self::PollAnswer => "poll_answer",
            Self::MyChatMember => "my_chat_member",
            Self::ChatMember => "chat_member",
            Self::ChatJoinRequest => "chat_join_request",
            Self::ChatBoost => "chat_boost",
            Self::RemovedChatBoost => "removed_chat_boost",
        })
    }
}
