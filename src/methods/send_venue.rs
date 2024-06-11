use crate::api::API;
use crate::entities::message::Message;
use crate::entities::misc::chat_id::ChatId;
use crate::entities::misc::reply_markup::ReplyMarkup;
use crate::entities::reply_parameters::ReplyParameters;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SendVenueParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl_into_future!(SendVenueRequest<'a>);

///Use this method to send information about a venue. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct SendVenueRequest<'a> {
    api: &'a API,
    params: SendVenueParams,
}

impl<'a> RequestT for SendVenueRequest<'a> {
    type ParamsType = SendVenueParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendVenue"
    }
    fn get_api_ref(&self) -> &API {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        false
    }
}
impl<'a> SendVenueRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        latitude: impl Into<f64>,
        longitude: impl Into<f64>,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            api,
            params: SendVenueParams {
                chat_id: chat_id.into(),
                latitude: latitude.into(),
                longitude: longitude.into(),
                title: title.into(),
                address: address.into(),
                business_connection_id: Option::default(),
                message_thread_id: Option::default(),
                foursquare_id: Option::default(),
                foursquare_type: Option::default(),
                google_place_id: Option::default(),
                google_place_type: Option::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
                message_effect_id: Option::default(),
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

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    #[must_use]
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = Some(message_thread_id.into());
        self
    }

    ///Latitude of the venue
    #[must_use]
    pub fn latitude(mut self, latitude: impl Into<f64>) -> Self {
        self.params.latitude = latitude.into();
        self
    }

    ///Longitude of the venue
    #[must_use]
    pub fn longitude(mut self, longitude: impl Into<f64>) -> Self {
        self.params.longitude = longitude.into();
        self
    }

    ///Name of the venue
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.params.title = title.into();
        self
    }

    ///Address of the venue
    #[must_use]
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.params.address = address.into();
        self
    }

    ///Foursquare identifier of the venue
    #[must_use]
    pub fn foursquare_id(mut self, foursquare_id: impl Into<String>) -> Self {
        self.params.foursquare_id = Some(foursquare_id.into());
        self
    }

    ///Foursquare type of the venue, if known. (For example, “arts\_entertainment/default”, “arts\_entertainment/aquarium” or “food/icecream”.)
    #[must_use]
    pub fn foursquare_type(mut self, foursquare_type: impl Into<String>) -> Self {
        self.params.foursquare_type = Some(foursquare_type.into());
        self
    }

    ///Google Places identifier of the venue
    #[must_use]
    pub fn google_place_id(mut self, google_place_id: impl Into<String>) -> Self {
        self.params.google_place_id = Some(google_place_id.into());
        self
    }

    ///Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)
    #[must_use]
    pub fn google_place_type(mut self, google_place_type: impl Into<String>) -> Self {
        self.params.google_place_type = Some(google_place_type.into());
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

    ///Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id(mut self, message_effect_id: impl Into<String>) -> Self {
        self.params.message_effect_id = Some(message_effect_id.into());
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

impl<'a> API {
    ///Use this method to send information about a venue. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_venue(
        &'a self,
        chat_id: impl Into<ChatId>,
        latitude: impl Into<f64>,
        longitude: impl Into<f64>,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> SendVenueRequest {
        SendVenueRequest::new(self, chat_id, latitude, longitude, title, address)
    }
}

// Divider: all content below this line will be preserved after code regen
