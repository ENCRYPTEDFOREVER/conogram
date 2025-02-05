


use serde::Serialize;

use crate::{
    api::Api,
    entities::{
        inline_keyboard_markup::InlineKeyboardMarkup, message::Message, misc::chat_id::ChatId,
    },
    
    impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]

pub struct StopMessageLiveLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl_into_future!(StopMessageLiveLocationRequest<'a>);

///Use this method to stop updating a live location message before *live\_period* expires. On success, if the message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
#[derive(Clone)]
pub struct StopMessageLiveLocationRequest<'a> {
    api: &'a Api,
    params: StopMessageLiveLocationParams,
}

impl RequestT for StopMessageLiveLocationRequest<'_> {
    type ParamsType = StopMessageLiveLocationParams;
    type ReturnType = Option<Message>;
    fn get_name() -> &'static str {
        "stopMessageLiveLocation"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> StopMessageLiveLocationRequest<'a> {
    pub fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: StopMessageLiveLocationParams {
                business_connection_id: Option::default(),
                chat_id: Option::default(),
                message_id: Option::default(),
                inline_message_id: Option::default(),
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

    ///Required if *inline\_message\_id* is not specified. Identifier of the message with live location to stop
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

    ///A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    #[must_use]
    pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Api {
    ///Use this method to stop updating a live location message before *live\_period* expires. On success, if the message is not an inline message, the edited [Message](https://core.telegram.org/bots/api/#message) is returned, otherwise *True* is returned.
    pub fn stop_message_live_location(&self) -> StopMessageLiveLocationRequest {
        StopMessageLiveLocationRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
