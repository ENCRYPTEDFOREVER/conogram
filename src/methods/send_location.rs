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
pub struct SendLocationParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl_into_future!(SendLocationRequest<'a>);

///Use this method to send point on the map. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct SendLocationRequest<'a> {
    api: &'a API,
    params: SendLocationParams,
}

impl<'a> RequestT for SendLocationRequest<'a> {
    type ParamsType = SendLocationParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendLocation"
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
impl<'a> SendLocationRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        latitude: impl Into<f64>,
        longitude: impl Into<f64>,
    ) -> Self {
        Self {
            api,
            params: SendLocationParams {
                chat_id: chat_id.into(),
                latitude: latitude.into(),
                longitude: longitude.into(),
                message_thread_id: Option::default(),
                horizontal_accuracy: Option::default(),
                live_period: Option::default(),
                heading: Option::default(),
                proximity_alert_radius: Option::default(),
                disable_notification: bool::default(),
                protect_content: bool::default(),
                reply_parameters: Option::default(),
                reply_markup: Option::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = Some(message_thread_id.into());
        self
    }

    ///Latitude of the location
    pub fn latitude(mut self, latitude: impl Into<f64>) -> Self {
        self.params.latitude = latitude.into();
        self
    }

    ///Longitude of the location
    pub fn longitude(mut self, longitude: impl Into<f64>) -> Self {
        self.params.longitude = longitude.into();
        self
    }

    ///The radius of uncertainty for the location, measured in meters; 0-1500
    pub fn horizontal_accuracy(mut self, horizontal_accuracy: impl Into<f64>) -> Self {
        self.params.horizontal_accuracy = Some(horizontal_accuracy.into());
        self
    }

    ///Period in seconds for which the location will be updated (see [Live Locations](https://telegram.org/blog/live-locations), should be between 60 and 86400.
    pub fn live_period(mut self, live_period: impl Into<i64>) -> Self {
        self.params.live_period = Some(live_period.into());
        self
    }

    ///For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub fn heading(mut self, heading: impl Into<i64>) -> Self {
        self.params.heading = Some(heading.into());
        self
    }

    ///For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub fn proximity_alert_radius(mut self, proximity_alert_radius: impl Into<i64>) -> Self {
        self.params.proximity_alert_radius = Some(proximity_alert_radius.into());
        self
    }

    ///Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }

    ///Description of the message to reply to
    pub fn reply_parameters(mut self, reply_parameters: impl Into<ReplyParameters>) -> Self {
        self.params.reply_parameters = Some(reply_parameters.into());
        self
    }

    ///Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub fn reply_markup(mut self, reply_markup: impl Into<ReplyMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl<'a> API {
    ///Use this method to send point on the map. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_location(
        &'a self,
        chat_id: impl Into<ChatId>,
        latitude: impl Into<f64>,
        longitude: impl Into<f64>,
    ) -> SendLocationRequest {
        SendLocationRequest::new(self, chat_id.into(), latitude.into(), longitude.into())
    }
}

// Divider: all content below this line will be preserved after code regen
