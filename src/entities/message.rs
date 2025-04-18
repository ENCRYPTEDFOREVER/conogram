use serde::{Deserialize, Serialize};

use crate::{
    entities::{
        animation::Animation, audio::Audio, chat::Chat, chat_background::ChatBackground,
        chat_boost_added::ChatBoostAdded, chat_shared::ChatShared, contact::Contact, dice::Dice,
        document::Document, external_reply_info::ExternalReplyInfo,
        forum_topic_closed::ForumTopicClosed, forum_topic_created::ForumTopicCreated,
        forum_topic_edited::ForumTopicEdited, forum_topic_reopened::ForumTopicReopened, game::Game,
        general_forum_topic_hidden::GeneralForumTopicHidden,
        general_forum_topic_unhidden::GeneralForumTopicUnhidden, gift_info::GiftInfo,
        giveaway::Giveaway, giveaway_completed::GiveawayCompleted,
        giveaway_created::GiveawayCreated, giveaway_winners::GiveawayWinners,
        inline_keyboard_markup::InlineKeyboardMarkup, invoice::Invoice,
        link_preview_options::LinkPreviewOptions, location::Location,
        maybe_inaccessible_message::MaybeInaccessibleMessage,
        message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged,
        message_entity::MessageEntity, message_origin::MessageOrigin,
        misc::message_effects::MessageEffect, paid_media_info::PaidMediaInfo,
        paid_message_price_changed::PaidMessagePriceChanged, passport_data::PassportData,
        photo_size::PhotoSize, poll::Poll, proximity_alert_triggered::ProximityAlertTriggered,
        refunded_payment::RefundedPayment, sticker::Sticker, story::Story,
        successful_payment::SuccessfulPayment, text_quote::TextQuote,
        unique_gift_info::UniqueGiftInfo, user::User, users_shared::UsersShared, venue::Venue,
        video::Video, video_chat_ended::VideoChatEnded,
        video_chat_participants_invited::VideoChatParticipantsInvited,
        video_chat_scheduled::VideoChatScheduled, video_chat_started::VideoChatStarted,
        video_note::VideoNote, voice::Voice, web_app_data::WebAppData,
        write_access_allowed::WriteAccessAllowed,
    },
    utils::deserialize_utils::is_false,
};

/// This object represents a message.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#message)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    pub message_id: i64,

    /// *Optional*. Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// *Optional*. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,

    /// *Optional*. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field *from* contains a fake sender user in non-channel chats.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Box<Chat>>,

    /// *Optional*. If the sender of the message boosted the chat, the number of boosts added by the user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_boost_count: Option<i64>,

    /// *Optional*. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_business_bot: Option<User>,

    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    pub date: i64,

    /// *Optional*. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// Chat the message belongs to
    pub chat: Box<Chat>,

    /// *Optional*. Information about the original message for forwarded messages
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forward_origin: Option<MessageOrigin>,

    /// *Optional*. *True*, if the message is sent to a forum topic
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_topic_message: bool,

    /// *Optional*. *True*, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_automatic_forward: bool,

    /// *Optional*. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further *reply\_to\_message* fields even if it itself is a reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,

    /// *Optional*. Information about the message that is being replied to, which may come from another chat or forum topic
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<ExternalReplyInfo>,

    /// *Optional*. For replies that quote part of the original message, the quoted part of the message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<TextQuote>,

    /// *Optional*. For replies to a story, the original story
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_story: Option<Story>,

    /// *Optional*. Bot through which the message was sent
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,

    /// *Optional*. Date the message was last edited in Unix time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,

    /// *Optional*. *True*, if the message can't be forwarded
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_protected_content: bool,

    /// *Optional*. True, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_from_offline: bool,

    /// *Optional*. The unique identifier of a media message group this message belongs to
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,

    /// *Optional*. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,

    /// *Optional*. The number of Telegram Stars that were paid by the sender of the message to send it
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_star_count: Option<i64>,

    /// *Optional*. For text messages, the actual UTF-8 text of the message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// *Optional*. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<MessageEntity>,

    /// *Optional*. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    /// *Optional*. Unique identifier of the message effect added to the message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect_id: Option<MessageEffect>,

    /// *Optional*. Message is an animation, information about the animation. For backward compatibility, when this field is set, the *document* field will also be set
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    /// *Optional*. Message is an audio file, information about the file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    /// *Optional*. Message is a general file, information about the file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    /// *Optional*. Message contains paid media; information about the paid media
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,

    /// *Optional*. Message is a photo, available sizes of the photo
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub photo: Vec<PhotoSize>,

    /// *Optional*. Message is a sticker, information about the sticker
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    /// *Optional*. Message is a forwarded story
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,

    /// *Optional*. Message is a video, information about the video
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    /// *Optional*. Message is a [video note](https://telegram.org/blog/video-messages-and-telescope), information about the video message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,

    /// *Optional*. Message is a voice message, information about the file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,

    /// *Optional*. Caption for the animation, audio, document, paid media, photo, video or voice
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// *Optional*. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,

    /// *Optional*. True, if the caption must be shown above the message media
    #[serde(default, skip_serializing_if = "is_false")]
    pub show_caption_above_media: bool,

    /// *Optional*. *True*, if the message media is covered by a spoiler animation
    #[serde(default, skip_serializing_if = "is_false")]
    pub has_media_spoiler: bool,

    /// *Optional*. Message is a shared contact, information about the contact
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    /// *Optional*. Message is a dice with random value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,

    /// *Optional*. Message is a game, information about the game. [More about games »](https://core.telegram.org/bots/api/#games)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,

    /// *Optional*. Message is a native poll, information about the poll
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    /// *Optional*. Message is a venue, information about the venue. For backward compatibility, when this field is set, the *location* field will also be set
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,

    /// *Optional*. Message is a shared location, information about the location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// *Optional*. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub new_chat_members: Vec<User>,

    /// *Optional*. A member was removed from the group, information about them (this member may be the bot itself)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,

    /// *Optional*. A chat title was changed to this value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,

    /// *Optional*. A chat photo was change to this value
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub new_chat_photo: Vec<PhotoSize>,

    /// *Optional*. Service message: the chat photo was deleted
    #[serde(default, skip_serializing_if = "is_false")]
    pub delete_chat_photo: bool,

    /// *Optional*. Service message: the group has been created
    #[serde(default, skip_serializing_if = "is_false")]
    pub group_chat_created: bool,

    /// *Optional*. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply\_to\_message if someone replies to a very first message in a directly created supergroup.
    #[serde(default, skip_serializing_if = "is_false")]
    pub supergroup_chat_created: bool,

    /// *Optional*. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply\_to\_message if someone replies to a very first message in a channel.
    #[serde(default, skip_serializing_if = "is_false")]
    pub channel_chat_created: bool,

    /// *Optional*. Service message: auto-delete timer settings changed in the chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,

    /// *Optional*. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    /// *Optional*. The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,

    /// *Optional*. Specified message was pinned. Note that the Message object in this field will not contain further *reply\_to\_message* fields even if it itself is a reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,

    /// *Optional*. Message is an invoice for a [payment](https://core.telegram.org/bots/api/#payments), information about the invoice. [More about payments »](https://core.telegram.org/bots/api/#payments)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,

    /// *Optional*. Message is a service message about a successful payment, information about the payment. [More about payments »](https://core.telegram.org/bots/api/#payments)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,

    /// *Optional*. Message is a service message about a refunded payment, information about the payment. [More about payments »](https://core.telegram.org/bots/api/#payments)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refunded_payment: Option<RefundedPayment>,

    /// *Optional*. Service message: users were shared with the bot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users_shared: Option<UsersShared>,

    /// *Optional*. Service message: a chat was shared with the bot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_shared: Option<ChatShared>,

    /// *Optional*. Service message: a regular gift was sent or received
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gift: Option<GiftInfo>,

    /// *Optional*. Service message: a unique gift was sent or received
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_gift: Option<UniqueGiftInfo>,

    /// *Optional*. The domain name of the website on which the user has logged in. [More about Telegram Login »](https://core.telegram.org/widgets/login)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,

    /// *Optional*. Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<WriteAccessAllowed>,

    /// *Optional*. Telegram Passport data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,

    /// *Optional*. Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,

    /// *Optional*. Service message: user boosted the chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost_added: Option<ChatBoostAdded>,

    /// *Optional*. Service message: chat background set
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_background_set: Option<ChatBackground>,

    /// *Optional*. Service message: forum topic created
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<ForumTopicCreated>,

    /// *Optional*. Service message: forum topic edited
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<ForumTopicEdited>,

    /// *Optional*. Service message: forum topic closed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<ForumTopicClosed>,

    /// *Optional*. Service message: forum topic reopened
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<ForumTopicReopened>,

    /// *Optional*. Service message: the 'General' forum topic hidden
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,

    /// *Optional*. Service message: the 'General' forum topic unhidden
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,

    /// *Optional*. Service message: a scheduled giveaway was created
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub giveaway_created: Option<GiveawayCreated>,

    /// *Optional*. The message is a scheduled giveaway message
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,

    /// *Optional*. A giveaway with public winners was completed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,

    /// *Optional*. Service message: a giveaway without public winners was completed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<GiveawayCompleted>,

    /// *Optional*. Service message: the price for paid messages has changed in the chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_message_price_changed: Option<PaidMessagePriceChanged>,

    /// *Optional*. Service message: video chat scheduled
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<VideoChatScheduled>,

    /// *Optional*. Service message: video chat started
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<VideoChatStarted>,

    /// *Optional*. Service message: video chat ended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<VideoChatEnded>,

    /// *Optional*. Service message: new participants invited to a video chat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,

    /// *Optional*. Service message: data sent by a Web App
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,

    /// *Optional*. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

// Divider: all content below this line will be preserved after code regen

use std::ops::Range;

use super::{
    input_media::InputMedia,
    misc::{formatting::FormattedText, input_file::InputFile},
    reaction_type::ReactionType,
    reply_parameters::ReplyParameters,
};
use crate::{
    api::Api,
    entities::misc::chat_id::ChatId,
    errors::ConogramError,
    methods::{
        copy_message::CopyMessageRequest, delete_message::DeleteMessageRequest,
        edit_message_media::EditMessageMediaRequest,
        edit_message_reply_markup::EditMessageReplyMarkupRequest,
        edit_message_text::EditMessageTextRequest, forward_message::ForwardMessageRequest,
        get_custom_emoji_stickers::GetCustomEmojiStickersRequest,
        pin_chat_message::PinChatMessageRequest, send_document::SendDocumentRequest,
        send_media_group::SendMediaGroupRequest, send_message::SendMessageRequest,
        send_photo::SendPhotoRequest, send_sticker::SendStickerRequest,
        send_video::SendVideoRequest, set_message_reaction::SetMessageReactionRequest,
        unpin_chat_message::UnpinChatMessageRequest,
    },
};

pub enum InputMessageText {
    String(String),
    FormattedText(FormattedText),
}

impl From<&str> for InputMessageText {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for InputMessageText {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<FormattedText> for InputMessageText {
    fn from(value: FormattedText) -> Self {
        Self::FormattedText(value)
    }
}

impl From<MaybeInaccessibleMessage> for Option<Message> {
    fn from(value: MaybeInaccessibleMessage) -> Self {
        match value {
            MaybeInaccessibleMessage::Message(m) => Some(m),
            MaybeInaccessibleMessage::InaccessibleMessage(_) => None,
        }
    }
}

impl<'a> From<&'a MaybeInaccessibleMessage> for Option<&'a Message> {
    fn from(value: &'a MaybeInaccessibleMessage) -> Self {
        match value {
            MaybeInaccessibleMessage::Message(m) => Some(m),
            MaybeInaccessibleMessage::InaccessibleMessage(_) => None,
        }
    }
}

impl Message {
    /// Static version of [``Message::get_url()``](Message::get_url())
    pub fn make_url(chat_id: impl Into<ChatId>, message_id: impl Into<i64>) -> String {
        match chat_id.into() {
            ChatId::Username(username) => {
                format!("https://t.me/{username}/{}", message_id.into())
            }
            ChatId::Id(id) => format!(
                "https://t.me/c/{}/{}",
                -id - 1000000000000,
                message_id.into()
            ),
        }
    }

    #[must_use]
    pub fn get_url(&self) -> String {
        if let Some(username) = &self.chat.username {
            format!("https://t.me/{username}/{}", self.message_id)
        } else {
            format!(
                "https://t.me/c/{}/{}",
                -self.chat.id - 1000000000000,
                self.message_id
            )
        }
    }

    /// ID of the message author
    #[must_use]
    pub fn from_id(&self) -> i64 {
        if let Some(sender_chat) = &self.sender_chat {
            sender_chat.id
        } else if let Some(sender_bot) = &self.sender_business_bot {
            sender_bot.id
        } else if let Some(from_user) = &self.from {
            from_user.id
        } else {
            0
        }
    }

    /// Returns `text` or `caption` if `text` is empty
    #[must_use]
    pub const fn get_text(&self) -> &Option<String> {
        if self.text.is_some() {
            &self.text
        } else {
            &self.caption
        }
    }

    /// Returns `entities` or `caption_entities` if `entities` is empty
    #[must_use]
    pub fn get_entities(&self) -> &Vec<MessageEntity> {
        if !self.entities.is_empty() {
            &self.entities
        } else {
            &self.caption_entities
        }
    }

    /// Returns vec of `custom_emoji_id` present in the message
    pub fn get_custom_emoji_ids(&self) -> Vec<String> {
        self.get_entities()
            .iter()
            .filter_map(|ent| ent.custom_emoji_id.as_ref())
            .map(String::from)
            .collect()
    }

    pub fn get_custom_emoji_stickers<'a>(
        &'a self,
        api: &'a Api,
    ) -> GetCustomEmojiStickersRequest<'a> {
        api.get_custom_emoji_stickers(self.get_custom_emoji_ids())
    }

    #[must_use]
    pub fn file_uid(&self) -> Option<String> {
        if let Some(m) = self.photo.first() {
            Some(m.file_unique_id.clone())
        } else if let Some(m) = &self.animation {
            Some(m.file_unique_id.clone())
        } else if let Some(m) = &self.audio {
            Some(m.file_unique_id.clone())
        } else if let Some(m) = &self.document {
            Some(m.file_unique_id.clone())
        } else if let Some(m) = &self.video {
            Some(m.file_unique_id.clone())
        } else if let Some(m) = &self.video_note {
            Some(m.file_unique_id.clone())
        } else if let Some(m) = &self.voice {
            Some(m.file_unique_id.clone())
        } else {
            self.sticker.as_ref().map(|m| m.file_unique_id.clone())
        }
    }

    #[must_use]
    pub fn file_id(&self) -> Option<String> {
        if let Some(m) = self.photo.first() {
            Some(m.file_id.clone())
        } else if let Some(m) = &self.animation {
            Some(m.file_id.clone())
        } else if let Some(m) = &self.audio {
            Some(m.file_id.clone())
        } else if let Some(m) = &self.document {
            Some(m.file_id.clone())
        } else if let Some(m) = &self.video {
            Some(m.file_id.clone())
        } else if let Some(m) = &self.video_note {
            Some(m.file_id.clone())
        } else if let Some(m) = &self.voice {
            Some(m.file_id.clone())
        } else {
            self.sticker.as_ref().map(|m| m.file_id.clone())
        }
    }

    #[must_use]
    pub fn get_formatted_text(&self) -> Option<FormattedText> {
        if let (Some(text), entities) = (self.get_text(), self.get_entities()) {
            Some(FormattedText::with_text(text.clone(), entities.clone()))
        } else {
            None
        }
    }

    /// Quote entire message and reply in the same Chat
    pub fn quote_reply<'a>(
        &'a self,
        api: &'a Api,
        text: impl Into<String>,
    ) -> SendMessageRequest<'a> {
        self.quote_reply_args(api, text, Option::<Range<usize>>::None, Option::<i64>::None)
    }

    /// Quote part of the message and reply in the same Chat
    pub fn quote_reply_partial<'a>(
        &'a self,
        api: &'a Api,
        text: impl Into<String>,
        quoting_range: impl Into<Range<usize>>,
    ) -> SendMessageRequest<'a> {
        self.quote_reply_args(api, text, Some(quoting_range), Option::<i64>::None)
    }

    /// Quote entire message and reply in the Chat, specified by `chat_id`
    pub fn quote_reply_to<'a>(
        &'a self,
        api: &'a Api,
        text: impl Into<String>,
        chat_id: impl Into<ChatId>,
    ) -> SendMessageRequest<'a> {
        self.quote_reply_args(api, text, Option::<Range<usize>>::None, Some(chat_id))
    }

    /// Quote part of the message and reply in the Chat, specified by `chat_id`
    pub fn quote_reply_partial_to<'a>(
        &'a self,
        api: &'a Api,
        text: impl Into<String>,
        quoting_range: impl Into<Range<usize>>,
        chat_id: impl Into<ChatId>,
    ) -> SendMessageRequest<'a> {
        self.quote_reply_args(api, text, Some(quoting_range), Some(chat_id))
    }

    /// Params:
    ///
    /// `chat_id`: identifier of the target chat
    ///
    /// `quoting_range`: char-range of the part of the original message which needs to be quoted
    pub fn quote_reply_args<'a>(
        &'a self,
        api: &'a Api,
        text: impl Into<String>,
        quoting_range: Option<impl Into<Range<usize>>>,
        chat_id: Option<impl Into<ChatId>>,
    ) -> SendMessageRequest<'a> {
        let chat_id = if let Some(chat_id) = chat_id {
            chat_id.into()
        } else {
            self.chat.id.into()
        };

        let (quote_text, quote_entities, quote_pos) = if let Some(range) = quoting_range {
            let range = range.into();
            let range_start = range.start as i64;

            let (quote_text, quote_entities) = self
                .get_formatted_text()
                .unwrap_or_default()
                .slice(range)
                .build();

            (Some(quote_text), quote_entities, Some(range_start))
        } else {
            (None, vec![], None)
        };

        let mut req =
            SendMessageRequest::new(api, chat_id, text).reply_parameters(ReplyParameters {
                message_id: self.message_id,
                chat_id: Some(self.chat.id.into()),
                quote: quote_text,
                quote_entities,
                quote_position: quote_pos,
                ..Default::default()
            });

        if self.is_topic_message {
            if let Some(thread_id) = self.message_thread_id {
                req = req.message_thread_id(thread_id);
            }
        }

        req
    }

    pub fn reply<'a>(
        &'a self,
        api: &'a Api,
        text: impl Into<InputMessageText>,
    ) -> SendMessageRequest<'a> {
        let mut req = match text.into() {
            InputMessageText::String(v) => api
                .send_message(self.chat.id, v)
                .reply_parameters(ReplyParameters::new_current_chat(self.message_id)),
            InputMessageText::FormattedText(ft) => self.reply_formatted(api, ft),
        };

        if self.is_topic_message {
            if let Some(thread_id) = self.message_thread_id {
                req = req.message_thread_id(thread_id);
            }
        }

        req
    }

    /// The same as `Message::reply().entities()`
    pub fn reply_entities<'a>(
        &'a self,
        api: &'a Api,
        text: impl Into<InputMessageText>,
        entities: impl IntoIterator<Item = MessageEntity>,
    ) -> SendMessageRequest<'a> {
        self.reply(api, text).entities(entities)
    }

    pub fn reply_formatted<'a>(
        &'a self,
        api: &'a Api,
        formatted_text: FormattedText,
    ) -> SendMessageRequest<'a> {
        let (text, entities) = formatted_text.build();
        self.reply(api, text).entities(entities)
    }

    /// Sends message to the same chat and thread
    pub fn answer<'a>(&'a self, api: &'a Api, text: impl Into<String>) -> SendMessageRequest<'a> {
        let mut req = api.send_message(self.chat.id, text);

        if self.is_topic_message {
            if let Some(thread_id) = self.message_thread_id {
                req = req.message_thread_id(thread_id);
            }
        }

        req
    }

    /// The same as `Message::answer().entities()`
    pub fn answer_entities<'a>(
        &'a self,
        api: &'a Api,
        text: impl Into<String>,
        entities: impl IntoIterator<Item = MessageEntity>,
    ) -> SendMessageRequest<'a> {
        self.answer(api, text).entities(entities)
    }

    pub fn unpin<'a>(&'a self, api: &'a Api) -> UnpinChatMessageRequest<'a> {
        api.unpin_chat_message(self.chat.id)
    }

    pub fn pin<'a>(&'a self, api: &'a Api) -> PinChatMessageRequest<'a> {
        api.pin_chat_message(self.chat.id, self.message_id)
    }

    pub fn forward<'a>(
        &'a self,
        api: &'a Api,
        to_chat_id: impl Into<ChatId>,
    ) -> ForwardMessageRequest<'a> {
        api.forward_message(to_chat_id, self.chat.id, self.message_id)
    }

    pub fn edit_media<'a>(
        &'a self,
        api: &'a Api,
        media: impl Into<InputMedia>,
    ) -> EditMessageMediaRequest<'a> {
        api.edit_message_media(media)
            .chat_id(self.chat.id)
            .message_id(self.message_id)
    }

    pub fn edit_text<'a>(
        &'a self,
        api: &'a Api,
        text: impl Into<String>,
    ) -> EditMessageTextRequest<'a> {
        api.edit_message_text(text.into())
            .message_id(self.message_id)
            .chat_id(self.chat.id)
    }

    pub fn edit_text_formatted<'a>(
        &'a self,
        api: &'a Api,
        ft: impl Into<FormattedText>,
    ) -> EditMessageTextRequest<'a> {
        let (text, entities) = ft.into().build();
        self.edit_text(api, text).entities(entities)
    }

    pub fn copy<'a>(&'a self, api: &'a Api, chat_id: impl Into<ChatId>) -> CopyMessageRequest<'a> {
        api.copy_message(chat_id, self.chat.id, self.message_id)
    }

    pub fn edit_reply_markup<'a>(&'a self, api: &'a Api) -> EditMessageReplyMarkupRequest<'a> {
        api.edit_message_reply_markup()
            .message_id(self.message_id)
            .chat_id(self.chat.id)
    }

    pub fn delete_reply_markup<'a>(&'a self, api: &'a Api) -> EditMessageReplyMarkupRequest<'a> {
        api.edit_message_reply_markup()
            .message_id(self.message_id)
            .chat_id(self.chat.id)
            .reply_markup(InlineKeyboardMarkup::empty())
    }

    pub fn delete<'a>(&'a self, api: &'a Api) -> DeleteMessageRequest<'a> {
        api.delete_message(self.chat.id, self.message_id)
    }

    /// Internal conogram method. Returns `Ok(false)` instead of `Err` if the message can't be deleted
    pub async fn delete_exp<'a>(&'a self, api: &'a Api) -> Result<bool, ConogramError> {
        api.delete_message_exp(self.chat.id, self.message_id).await
    }

    pub fn reply_photo<'a>(
        &'a self,
        api: &'a Api,
        photo: impl Into<InputFile>,
    ) -> SendPhotoRequest<'a> {
        let mut req = api
            .send_photo(self.chat.id, photo)
            .reply_parameters(ReplyParameters::new_current_chat(self.message_id));

        if self.is_topic_message {
            if let Some(thread_id) = self.message_thread_id {
                req = req.message_thread_id(thread_id);
            }
        }
        req
    }

    pub fn reply_media_group<'a>(
        &'a self,
        api: &'a Api,
        media: impl IntoIterator<Item = impl Into<InputMedia>>,
    ) -> SendMediaGroupRequest<'a> {
        let mut req = api
            .send_media_group(self.chat.id, media)
            .reply_parameters(ReplyParameters::new_current_chat(self.message_id));

        if self.is_topic_message {
            if let Some(thread_id) = self.message_thread_id {
                req = req.message_thread_id(thread_id);
            }
        }
        req
    }

    pub fn reply_video<'a>(
        &'a self,
        api: &'a Api,
        video: impl Into<InputFile>,
    ) -> SendVideoRequest<'a> {
        let mut req = api
            .send_video(self.chat.id, video)
            .reply_parameters(ReplyParameters::new_current_chat(self.message_id));

        if self.is_topic_message {
            if let Some(thread_id) = self.message_thread_id {
                req = req.message_thread_id(thread_id);
            }
        }
        req
    }

    pub fn reply_document<'a>(
        &'a self,
        api: &'a Api,
        document: impl Into<InputFile>,
    ) -> SendDocumentRequest<'a> {
        let mut req = api
            .send_document(self.chat.id, document)
            .reply_parameters(ReplyParameters::new_current_chat(self.message_id));

        if self.is_topic_message {
            if let Some(thread_id) = self.message_thread_id {
                req = req.message_thread_id(thread_id);
            }
        }
        req
    }

    pub fn reply_sticker<'a>(
        &'a self,
        api: &'a Api,
        sticker: impl Into<InputFile>,
    ) -> SendStickerRequest<'a> {
        let mut req = api
            .send_sticker(self.chat.id, sticker)
            .reply_parameters(ReplyParameters::new_current_chat(self.message_id));

        if self.is_topic_message {
            if let Some(thread_id) = self.message_thread_id {
                req = req.message_thread_id(thread_id);
            }
        }

        req
    }

    pub fn copy_to<'a>(
        &'a self,
        api: &'a Api,
        chat_id: impl Into<ChatId>,
    ) -> CopyMessageRequest<'a> {
        api.copy_message(chat_id, self.chat.id, self.message_id)
    }

    pub fn set_reactions<'a>(
        &'a self,
        api: &'a Api,
        reactions: impl IntoIterator<Item = impl Into<ReactionType>>,
    ) -> SetMessageReactionRequest<'a> {
        api.set_message_reaction(self.chat.id, self.message_id)
            .reaction(reactions)
    }

    pub fn delete_reactions<'a>(&'a self, api: &'a Api) -> SetMessageReactionRequest<'a> {
        let reactions: [ReactionType; 0] = [];
        self.set_reactions(api, reactions)
    }

    /// The same as [`message.set_reactions([reaction])`](Self::set_reactions)
    pub fn react<'a>(
        &'a self,
        api: &'a Api,
        reaction: impl Into<ReactionType>,
    ) -> SetMessageReactionRequest<'a> {
        self.set_reactions(api, [reaction.into()])
    }
}
