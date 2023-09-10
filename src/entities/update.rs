use crate::entities::callback_query::CallbackQuery;
use crate::entities::chat_join_request::ChatJoinRequest;
use crate::entities::chat_member_updated::ChatMemberUpdated;
use crate::entities::chosen_inline_result::ChosenInlineResult;
use crate::entities::inline_query::InlineQuery;
use crate::entities::message::Message;
use crate::entities::poll::Poll;
use crate::entities::poll_answer::PollAnswer;
use crate::entities::pre_checkout_query::PreCheckoutQuery;
use crate::entities::shipping_query::ShippingQuery;
use crate::utils::deserialize_utils::deserialize_boxed_option;
use serde::{Deserialize, Serialize};

///This [object](https://core.telegram.org/bots/api/#available-types) represents an incoming update.  
///At most **one** of the optional parameters can be present in any given update.
///API Reference: [link](https://core.telegram.org/bots/api/#update)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Update {
    ///The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you're using [webhooks](https://core.telegram.org/bots/api/#setwebhook), since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,

    ///*Optional*. New incoming message of any kind - text, photo, sticker, etc.
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub message: Option<Box<Message>>,

    ///*Optional*. New version of a message that is known to the bot and was edited
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub edited_message: Option<Box<Message>>,

    ///*Optional*. New incoming channel post of any kind - text, photo, sticker, etc.
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub channel_post: Option<Box<Message>>,

    ///*Optional*. New version of a channel post that is known to the bot and was edited
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub edited_channel_post: Option<Box<Message>>,

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

    ///*Optional*. New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    ///*Optional*. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,

    ///*Optional*. The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,

    ///*Optional*. A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify “chat\_member” in the list of *allowed\_updates* to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,

    ///*Optional*. A request to join the chat has been sent. The bot must have the *can\_invite\_users* administrator right in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,
}
// Divider: all content below this line will be preserved after code regen

pub enum AllowedUpdates {
    Message,
    EditedMessage,

    ChannelPost,
    EditedChannelPost,

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
}

impl AllowedUpdates {
    /// All existing updates
    pub fn all() -> Vec<Self> {
        vec![
            Self::Message,
            Self::EditedMessage,
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
        ]
    }
}

impl ToString for AllowedUpdates {
    fn to_string(&self) -> String {
        match self {
            AllowedUpdates::Message => "message".into(),
            AllowedUpdates::EditedMessage => "edited_message".into(),
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
        }
    }
}
