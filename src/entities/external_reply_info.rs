use crate::entities::animation::Animation;
use crate::entities::audio::Audio;
use crate::entities::chat::Chat;
use crate::entities::contact::Contact;
use crate::entities::dice::Dice;
use crate::entities::document::Document;
use crate::entities::game::Game;
use crate::entities::giveaway::Giveaway;
use crate::entities::giveaway_winners::GiveawayWinners;
use crate::entities::invoice::Invoice;
use crate::entities::link_preview_options::LinkPreviewOptions;
use crate::entities::location::Location;
use crate::entities::message_origin::MessageOrigin;
use crate::entities::paid_media_info::PaidMediaInfo;
use crate::entities::photo_size::PhotoSize;
use crate::entities::poll::Poll;
use crate::entities::sticker::Sticker;
use crate::entities::story::Story;
use crate::entities::venue::Venue;
use crate::entities::video::Video;
use crate::entities::video_note::VideoNote;
use crate::entities::voice::Voice;
use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///This object contains information about a message that is being replied to, which may come from another chat or forum topic.
///API Reference: [link](https://core.telegram.org/bots/api/#externalreplyinfo)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ExternalReplyInfo {
    ///Origin of the message replied to by the given message
    pub origin: MessageOrigin,

    ///*Optional*. Chat the original message belongs to. Available only if the chat is a supergroup or a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Box<Chat>>,

    ///*Optional*. Unique message identifier inside the original chat. Available only if the original chat is a supergroup or a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    ///*Optional*. Options used for link preview generation for the original message, if it is a text message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    ///*Optional*. Message is an animation, information about the animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    ///*Optional*. Message is an audio file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    ///*Optional*. Message is a general file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    ///*Optional*. Message contains paid media; information about the paid media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,

    ///*Optional*. Message is a photo, available sizes of the photo
    #[serde(default)]
    pub photo: Vec<PhotoSize>,

    ///*Optional*. Message is a sticker, information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    ///*Optional*. Message is a forwarded story
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,

    ///*Optional*. Message is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    ///*Optional*. Message is a [video note](https://telegram.org/blog/video-messages-and-telescope), information about the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,

    ///*Optional*. Message is a voice message, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,

    ///*Optional*. *True*, if the message media is covered by a spoiler animation
    #[serde(default, skip_serializing_if = "is_false")]
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

    ///*Optional*. Message is a scheduled giveaway, information about the giveaway
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,

    ///*Optional*. A giveaway with public winners was completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,

    ///*Optional*. Message is an invoice for a [payment](https://core.telegram.org/bots/api/#payments), information about the invoice. [More about payments »](https://core.telegram.org/bots/api/#payments)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,

    ///*Optional*. Message is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    ///*Optional*. Message is a native poll, information about the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    ///*Optional*. Message is a venue, information about the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
}
// Divider: all content below this line will be preserved after code regen
