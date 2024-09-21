use crate::api::API;
use crate::entities::input_paid_media::InputPaidMedia;
use crate::entities::message::Message;
use crate::entities::message_entity::MessageEntity;
use crate::entities::misc::chat_id::ChatId;
use crate::entities::misc::input_file::GetFiles;
use crate::entities::misc::input_file::InputFile;
use crate::entities::misc::input_file::Moose;
use crate::entities::misc::reply_markup::ReplyMarkup;
use crate::entities::reply_parameters::ReplyParameters;
use crate::errors::ConogramError;
use crate::impl_into_future_multipart;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::collections::HashMap;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SendPaidMediaParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub star_count: i64,
    pub media: Vec<InputPaidMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub caption_entities: Vec<MessageEntity>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub show_caption_above_media: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl GetFiles for SendPaidMediaParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        for media in &self.media {
            map.extend(media.get_files());
        }
        map
    }
}
impl_into_future_multipart!(SendPaidMediaRequest<'a>);

///Use this method to send paid media. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct SendPaidMediaRequest<'a> {
    api: &'a API,
    params: SendPaidMediaParams,
}

impl<'a> RequestT for SendPaidMediaRequest<'a> {
    type ParamsType = SendPaidMediaParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendPaidMedia"
    }
    fn get_api_ref(&self) -> &API {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        true
    }
}
impl<'a> SendPaidMediaRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        star_count: impl Into<i64>,
        media: impl IntoIterator<Item = impl Into<InputPaidMedia>>,
    ) -> Self {
        Self {
            api,
            params: SendPaidMediaParams {
                chat_id: chat_id.into(),
                star_count: star_count.into(),
                media: media.into_iter().map(Into::into).collect(),
                business_connection_id: Option::default(),
                payload: Option::default(),
                caption: Option::default(),
                parse_mode: Option::default(),
                caption_entities: Vec::default(),
                show_caption_above_media: bool::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
                reply_parameters: Option::default(),
                reply_markup: Option::default(),
            },
        }
    }

    ///Unique identifier of the business connection on behalf of which the message will be sent
    #[must_use]
    pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(business_connection_id.into());
        self
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`). If the chat is a channel, all Telegram Star proceeds from this media will be credited to the chat's balance. Otherwise, they will be credited to the bot's balance.
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///The number of Telegram Stars that must be paid to buy access to the media; 1-2500
    #[must_use]
    pub fn star_count(mut self, star_count: impl Into<i64>) -> Self {
        self.params.star_count = star_count.into();
        self
    }

    ///A JSON-serialized array describing the media to be sent; up to 10 items
    #[must_use]
    pub fn media(mut self, media: impl IntoIterator<Item = impl Into<InputPaidMedia>>) -> Self {
        self.params.media = media.into_iter().map(Into::into).collect();
        self
    }

    ///Bot-defined paid media payload, 0-128 bytes. This will not be displayed to the user, use it for your internal processes.
    #[must_use]
    pub fn payload(mut self, payload: impl Into<String>) -> Self {
        self.params.payload = Some(payload.into());
        self
    }

    ///Media caption, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption(mut self, caption: impl Into<String>) -> Self {
        self.params.caption = Some(caption.into());
        self
    }

    ///Mode for parsing entities in the media caption. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details.
    #[must_use]
    pub fn parse_mode(mut self, parse_mode: impl Into<String>) -> Self {
        self.params.parse_mode = Some(parse_mode.into());
        self
    }

    ///A JSON-serialized list of special entities that appear in the caption, which can be specified instead of *parse\_mode*
    #[must_use]
    pub fn caption_entities(
        mut self,
        caption_entities: impl IntoIterator<Item = impl Into<MessageEntity>>,
    ) -> Self {
        self.params.caption_entities = caption_entities.into_iter().map(Into::into).collect();
        self
    }

    ///Pass *True*, if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media(mut self, show_caption_above_media: impl Into<bool>) -> Self {
        self.params.show_caption_above_media = show_caption_above_media.into();
        self
    }

    ///Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }

    ///Description of the message to reply to
    #[must_use]
    pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
        self.params.reply_parameters = Some(reply_parameters.into());
        self
    }

    ///Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove a reply keyboard or to force a reply from the user
    #[must_use]
    pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl API {
    ///Use this method to send paid media. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_paid_media(
        &self,
        chat_id: impl Into<ChatId>,
        star_count: impl Into<i64>,
        media: impl IntoIterator<Item = impl Into<InputPaidMedia>>,
    ) -> SendPaidMediaRequest {
        SendPaidMediaRequest::new(self, chat_id, star_count, media)
    }
}

// Divider: all content below this line will be preserved after code regen
