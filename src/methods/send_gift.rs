use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API, entities::message_entity::MessageEntity, errors::ConogramError, impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct SendGiftParams {
    pub user_id: i64,
    pub gift_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text_entities: Vec<MessageEntity>,
}

impl_into_future!(SendGiftRequest<'a>);

///Sends a gift to the given user. The gift can't be converted to Telegram Stars by the user. Returns *True* on success.
#[derive(Clone)]
pub struct SendGiftRequest<'a> {
    api: &'a API,
    params: SendGiftParams,
}

impl<'a> RequestT for SendGiftRequest<'a> {
    type ParamsType = SendGiftParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "sendGift"
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
impl<'a> SendGiftRequest<'a> {
    pub fn new(api: &'a API, user_id: impl Into<i64>, gift_id: impl Into<String>) -> Self {
        Self {
            api,
            params: SendGiftParams {
                user_id: user_id.into(),
                gift_id: gift_id.into(),
                text: Option::default(),
                text_parse_mode: Option::default(),
                text_entities: Vec::default(),
            },
        }
    }

    ///Unique identifier of the target user that will receive the gift
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Identifier of the gift
    #[must_use]
    pub fn gift_id(mut self, gift_id: impl Into<String>) -> Self {
        self.params.gift_id = gift_id.into();
        self
    }

    ///Text that will be shown along with the gift; 0-255 characters
    #[must_use]
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.params.text = Some(text.into());
        self
    }

    ///Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api/#formatting-options) for more details. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\_emoji” are ignored.
    #[must_use]
    pub fn text_parse_mode(mut self, text_parse_mode: impl Into<String>) -> Self {
        self.params.text_parse_mode = Some(text_parse_mode.into());
        self
    }

    ///A JSON-serialized list of special entities that appear in the gift text. It can be specified instead of *text\_parse\_mode*. Entities other than “bold”, “italic”, “underline”, “strikethrough”, “spoiler”, and “custom\_emoji” are ignored.
    #[must_use]
    pub fn text_entities(
        mut self,
        text_entities: impl IntoIterator<Item = impl Into<MessageEntity>>,
    ) -> Self {
        self.params.text_entities = text_entities.into_iter().map(Into::into).collect();
        self
    }
}

impl API {
    ///Sends a gift to the given user. The gift can't be converted to Telegram Stars by the user. Returns *True* on success.
    pub fn send_gift(
        &self,
        user_id: impl Into<i64>,
        gift_id: impl Into<String>,
    ) -> SendGiftRequest {
        SendGiftRequest::new(self, user_id, gift_id)
    }
}

// Divider: all content below this line will be preserved after code regen
