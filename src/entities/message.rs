use crate::entities::animation::Animation;
use crate::entities::audio::Audio;
use crate::entities::chat::Chat;
use crate::entities::chat_shared::ChatShared;
use crate::entities::contact::Contact;
use crate::entities::dice::Dice;
use crate::entities::document::Document;
use crate::entities::forum_topic_closed::ForumTopicClosed;
use crate::entities::forum_topic_created::ForumTopicCreated;
use crate::entities::forum_topic_edited::ForumTopicEdited;
use crate::entities::forum_topic_reopened::ForumTopicReopened;
use crate::entities::game::Game;
use crate::entities::general_forum_topic_hidden::GeneralForumTopicHidden;
use crate::entities::general_forum_topic_unhidden::GeneralForumTopicUnhidden;
use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::invoice::Invoice;
use crate::entities::location::Location;
use crate::entities::message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;
use crate::entities::message_entity::MessageEntity;
use crate::entities::passport_data::PassportData;
use crate::entities::photo_size::PhotoSize;
use crate::entities::poll::Poll;
use crate::entities::proximity_alert_triggered::ProximityAlertTriggered;
use crate::entities::sticker::Sticker;
use crate::entities::successful_payment::SuccessfulPayment;
use crate::entities::user::User;
use crate::entities::user_shared::UserShared;
use crate::entities::venue::Venue;
use crate::entities::video::Video;
use crate::entities::video_chat_ended::VideoChatEnded;
use crate::entities::video_chat_participants_invited::VideoChatParticipantsInvited;
use crate::entities::video_chat_scheduled::VideoChatScheduled;
use crate::entities::video_chat_started::VideoChatStarted;
use crate::entities::video_note::VideoNote;
use crate::entities::voice::Voice;
use crate::entities::web_app_data::WebAppData;
use crate::entities::write_access_allowed::WriteAccessAllowed;
use crate::utils::deserialize_utils::deserialize_boxed;
use crate::utils::deserialize_utils::deserialize_boxed_option;
use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///This object represents a message.
///API Reference: [link](https://core.telegram.org/bots/api/#message)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Message {
    ///Unique message identifier inside this chat
    pub message_id: i64,

    ///*Optional*. Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    ///*Optional*. Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,

    ///*Optional*. Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub sender_chat: Option<Box<Chat>>,

    ///Date the message was sent in Unix time
    pub date: i64,

    ///Conversation the message belongs to
    #[serde(deserialize_with = "deserialize_boxed")]
    pub chat: Box<Chat>,

    ///*Optional*. For forwarded messages, sender of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from: Option<User>,

    ///*Optional*. For messages forwarded from channels or from anonymous administrators, information about the original sender chat
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub forward_from_chat: Option<Box<Chat>>,

    ///*Optional*. For messages forwarded from channels, identifier of the original message in the channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_message_id: Option<i64>,

    ///*Optional*. For forwarded messages that were originally sent in channels or by an anonymous chat administrator, signature of the message sender if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_signature: Option<String>,

    ///*Optional*. Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_sender_name: Option<String>,

    ///*Optional*. For forwarded messages, date the original message was sent in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_date: Option<i64>,

    ///*Optional*. *True*, if the message is sent to a forum topic
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_topic_message: bool,

    ///*Optional*. *True*, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_automatic_forward: bool,

    ///*Optional*. For replies, the original message. Note that the Message object in this field will not contain further *reply\_to\_message* fields even if it itself is a reply.
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub reply_to_message: Option<Box<Message>>,

    ///*Optional*. Bot through which the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,

    ///*Optional*. Date the message was last edited in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,

    ///*Optional*. *True*, if the message can't be forwarded
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_protected_content: bool,

    ///*Optional*. The unique identifier of a media message group this message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,

    ///*Optional*. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,

    ///*Optional*. For text messages, the actual UTF-8 text of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    ///*Optional*. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[serde(default)]
    pub entities: Vec<MessageEntity>,

    ///*Optional*. Message is an animation, information about the animation. For backward compatibility, when this field is set, the *document* field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    ///*Optional*. Message is an audio file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    ///*Optional*. Message is a general file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    ///*Optional*. Message is a photo, available sizes of the photo
    #[serde(default)]
    pub photo: Vec<PhotoSize>,

    ///*Optional*. Message is a sticker, information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    ///*Optional*. Message is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    ///*Optional*. Message is a [video note](https://telegram.org/blog/video-messages-and-telescope), information about the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,

    ///*Optional*. Message is a voice message, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,

    ///*Optional*. Caption for the animation, audio, document, photo, video or voice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    ///*Optional*. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(default)]
    pub caption_entities: Vec<MessageEntity>,

    ///*Optional*. *True*, if the message media is covered by a spoiler animation
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_media_spoiler: bool,

    ///*Optional*. Message is a shared contact, information about the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    ///*Optional*. Message is a dice with random value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,

    ///*Optional*. Message is a game, information about the game. [More about games »](https://core.telegram.org/bots/api/#games)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,

    ///*Optional*. Message is a native poll, information about the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    ///*Optional*. Message is a venue, information about the venue. For backward compatibility, when this field is set, the *location* field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,

    ///*Optional*. Message is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    ///*Optional*. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[serde(default)]
    pub new_chat_members: Vec<User>,

    ///*Optional*. A member was removed from the group, information about them (this member may be the bot itself)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,

    ///*Optional*. A chat title was changed to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,

    ///*Optional*. A chat photo was change to this value
    #[serde(default)]
    pub new_chat_photo: Vec<PhotoSize>,

    ///*Optional*. Service message: the chat photo was deleted
    #[serde(skip_serializing_if = "is_false", default)]
    pub delete_chat_photo: bool,

    ///*Optional*. Service message: the group has been created
    #[serde(skip_serializing_if = "is_false", default)]
    pub group_chat_created: bool,

    ///*Optional*. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply\_to\_message if someone replies to a very first message in a directly created supergroup.
    #[serde(skip_serializing_if = "is_false", default)]
    pub supergroup_chat_created: bool,

    ///*Optional*. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply\_to\_message if someone replies to a very first message in a channel.
    #[serde(skip_serializing_if = "is_false", default)]
    pub channel_chat_created: bool,

    ///*Optional*. Service message: auto-delete timer settings changed in the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,

    ///*Optional*. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    ///*Optional*. The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,

    ///*Optional*. Specified message was pinned. Note that the Message object in this field will not contain further *reply\_to\_message* fields even if it is itself a reply.
    #[serde(
        deserialize_with = "deserialize_boxed_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub pinned_message: Option<Box<Message>>,

    ///*Optional*. Message is an invoice for a [payment](https://core.telegram.org/bots/api/#payments), information about the invoice. [More about payments »](https://core.telegram.org/bots/api/#payments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,

    ///*Optional*. Message is a service message about a successful payment, information about the payment. [More about payments »](https://core.telegram.org/bots/api/#payments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,

    ///*Optional*. Service message: a user was shared with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_shared: Option<UserShared>,

    ///*Optional*. Service message: a chat was shared with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_shared: Option<ChatShared>,

    ///*Optional*. The domain name of the website on which the user has logged in. [More about Telegram Login »](https://core.telegram.org/widgets/login)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,

    ///*Optional*. Service message: the user allowed the bot added to the attachment menu to write messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<WriteAccessAllowed>,

    ///*Optional*. Telegram Passport data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,

    ///*Optional*. Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,

    ///*Optional*. Service message: forum topic created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<ForumTopicCreated>,

    ///*Optional*. Service message: forum topic edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<ForumTopicEdited>,

    ///*Optional*. Service message: forum topic closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<ForumTopicClosed>,

    ///*Optional*. Service message: forum topic reopened
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<ForumTopicReopened>,

    ///*Optional*. Service message: the 'General' forum topic hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,

    ///*Optional*. Service message: the 'General' forum topic unhidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,

    ///*Optional*. Service message: video chat scheduled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<VideoChatScheduled>,

    ///*Optional*. Service message: video chat started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<VideoChatStarted>,

    ///*Optional*. Service message: video chat ended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<VideoChatEnded>,

    ///*Optional*. Service message: new participants invited to a video chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,

    ///*Optional*. Service message: data sent by a Web App
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,

    ///*Optional*. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
// Divider: all content below this line will be preserved after code regen

use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::methods::copy_message::CopyMessageRequest;
use crate::methods::delete_message::DeleteMessageRequest;
use crate::methods::edit_message_reply_markup::EditMessageReplyMarkupRequest;
use crate::methods::edit_message_text::EditMessageTextRequest;
use crate::methods::send_document::SendDocumentRequest;
use crate::methods::send_message::SendMessageRequest;

use super::misc::input_file::InputFile;

impl Message {
    pub fn reply<'a>(&'a self, api: &'a API, text: impl Into<String>) -> SendMessageRequest {
        api.send_message(self.chat.id, text)
            .reply_to_message_id(self.message_id)
    }

    // Sends message to the same chat and thread
    pub fn answer<'a>(&'a self, api: &'a API, text: impl Into<String>) -> SendMessageRequest {
        if self.is_topic_message {
            if let Some(thread_id) = self.message_thread_id {
                return api
                    .send_message(self.chat.id, text)
                    .message_thread_id(thread_id);
            }
        }
        api.send_message(self.chat.id, text)
    }

    pub fn edit_text<'a>(
        &'a self,
        api: &'a API,
        text: impl Into<String>,
    ) -> EditMessageTextRequest {
        api.edit_message_text(text.into())
            .message_id(self.message_id)
            .chat_id(self.chat.id)
    }

    pub fn edit_reply_markup<'a>(&'a self, api: &'a API) -> EditMessageReplyMarkupRequest {
        api.edit_message_reply_markup()
            .message_id(self.message_id)
            .chat_id(self.chat.id)
    }

    pub fn delete_reply_markup<'a>(&'a self, api: &'a API) -> EditMessageReplyMarkupRequest {
        api.edit_message_reply_markup()
            .message_id(self.message_id)
            .chat_id(self.chat.id)
            .reply_markup(InlineKeyboardMarkup::empty())
    }

    pub fn delete<'a>(&'a self, api: &'a API) -> DeleteMessageRequest {
        api.delete_message(self.chat.id, self.message_id)
    }

    pub fn reply_document<'a>(
        &'a self,
        api: &'a API,
        document: impl Into<InputFile>,
    ) -> SendDocumentRequest {
        api.send_document(self.chat.id, document)
            .reply_to_message_id(self.message_id)
    }

    pub fn copy_to<'a>(&'a self, api: &'a API, chat_id: impl Into<ChatId>) -> CopyMessageRequest {
        api.copy_message(chat_id, self.chat.id, self.message_id)
    }
}
