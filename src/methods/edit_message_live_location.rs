use crate::api::API;
use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::message::Message;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct EditMessageLiveLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl_into_future!(EditMessageLiveLocationRequest<'a>);

///Use this method to edit live location messages. A location can be edited until its *live\_period* expires or editing is explicitly disabled by a call to [stopMessageLiveLocation](https://core.telegram.org/bots/api/#stopmessagelivelocation). On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
#[derive(Clone)]
pub struct EditMessageLiveLocationRequest<'a> {
    api: &'a API,
    params: EditMessageLiveLocationParams,
}

impl<'a> RequestT for EditMessageLiveLocationRequest<'a> {
    type ParamsType = EditMessageLiveLocationParams;
    type ReturnType = Option<Message>;
    fn get_name() -> &'static str {
        "editMessageLiveLocation"
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
impl<'a> EditMessageLiveLocationRequest<'a> {
    pub fn new(api: &'a API, latitude: impl Into<f64>, longitude: impl Into<f64>) -> Self {
        Self {
            api,
            params: EditMessageLiveLocationParams {
                latitude: latitude.into(),
                longitude: longitude.into(),
                business_connection_id: Option::default(),
                chat_id: Option::default(),
                message_id: Option::default(),
                inline_message_id: Option::default(),
                live_period: Option::default(),
                horizontal_accuracy: Option::default(),
                heading: Option::default(),
                proximity_alert_radius: Option::default(),
                reply_markup: Option::default(),
            },
        }
    }

    ///Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[must_use]
    pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(business_connection_id.into());
        self
    }

    ///Required if *inline\_message\_id* is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = Some(chat_id.into());
        self
    }

    ///Required if *inline\_message\_id* is not specified. Identifier of the message to edit
    #[must_use]
    pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
        self.params.message_id = Some(message_id.into());
        self
    }

    ///Required if *chat\_id* and *message\_id* are not specified. Identifier of the inline message
    #[must_use]
    pub fn inline_message_id(mut self, inline_message_id: impl Into<String>) -> Self {
        self.params.inline_message_id = Some(inline_message_id.into());
        self
    }

    ///Latitude of new location
    #[must_use]
    pub fn latitude(mut self, latitude: impl Into<f64>) -> Self {
        self.params.latitude = latitude.into();
        self
    }

    ///Longitude of new location
    #[must_use]
    pub fn longitude(mut self, longitude: impl Into<f64>) -> Self {
        self.params.longitude = longitude.into();
        self
    }

    ///New period in seconds during which the location can be updated, starting from the message send date. If 0x7FFFFFFF is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current *live\_period* by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then *live\_period* remains unchanged
    #[must_use]
    pub fn live_period(mut self, live_period: impl Into<i64>) -> Self {
        self.params.live_period = Some(live_period.into());
        self
    }

    ///The radius of uncertainty for the location, measured in meters; 0-1500
    #[must_use]
    pub fn horizontal_accuracy(mut self, horizontal_accuracy: impl Into<f64>) -> Self {
        self.params.horizontal_accuracy = Some(horizontal_accuracy.into());
        self
    }

    ///Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading(mut self, heading: impl Into<i64>) -> Self {
        self.params.heading = Some(heading.into());
        self
    }

    ///The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius(mut self, proximity_alert_radius: impl Into<i64>) -> Self {
        self.params.proximity_alert_radius = Some(proximity_alert_radius.into());
        self
    }

    ///A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    #[must_use]
    pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl<'a> API {
    ///Use this method to edit live location messages. A location can be edited until its *live\_period* expires or editing is explicitly disabled by a call to [stopMessageLiveLocation](https://core.telegram.org/bots/api/#stopmessagelivelocation). On success, if the edited message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    pub fn edit_message_live_location(
        &'a self,
        latitude: impl Into<f64>,
        longitude: impl Into<f64>,
    ) -> EditMessageLiveLocationRequest {
        EditMessageLiveLocationRequest::new(self, latitude, longitude)
    }
}

// Divider: all content below this line will be preserved after code regen
