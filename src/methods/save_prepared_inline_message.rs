use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api,
    entities::{
        inline_query_result::InlineQueryResult, prepared_inline_message::PreparedInlineMessage,
    },
    errors::ConogramError,
    impl_into_future,
    request::RequestT,
    utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]
pub struct SavePreparedInlineMessageParams {
    pub user_id: i64,
    pub result: InlineQueryResult,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_user_chats: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_bot_chats: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_group_chats: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_channel_chats: bool,
}

impl_into_future!(SavePreparedInlineMessageRequest<'a>);

///Stores a message that can be sent by a user of a Mini App. Returns a [PreparedInlineMessage](https://core.telegram.org/bots/api/#preparedinlinemessage) object.
#[derive(Clone)]
pub struct SavePreparedInlineMessageRequest<'a> {
    api: &'a Api,
    params: SavePreparedInlineMessageParams,
}

impl RequestT for SavePreparedInlineMessageRequest<'_> {
    type ParamsType = SavePreparedInlineMessageParams;
    type ReturnType = PreparedInlineMessage;
    fn get_name() -> &'static str {
        "savePreparedInlineMessage"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        false
    }
}
impl<'a> SavePreparedInlineMessageRequest<'a> {
    pub fn new(
        api: &'a Api,
        user_id: impl Into<i64>,
        result: impl Into<InlineQueryResult>,
    ) -> Self {
        Self {
            api,
            params: SavePreparedInlineMessageParams {
                user_id: user_id.into(),
                result: result.into(),
                allow_user_chats: bool::default(),
                allow_bot_chats: bool::default(),
                allow_group_chats: bool::default(),
                allow_channel_chats: bool::default(),
            },
        }
    }

    ///Unique identifier of the target user that can use the prepared message
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///A JSON-serialized object describing the message to be sent
    #[must_use]
    pub fn result(mut self, result: impl Into<InlineQueryResult>) -> Self {
        self.params.result = result.into();
        self
    }

    ///Pass *True* if the message can be sent to private chats with users
    #[must_use]
    pub fn allow_user_chats(mut self, allow_user_chats: impl Into<bool>) -> Self {
        self.params.allow_user_chats = allow_user_chats.into();
        self
    }

    ///Pass *True* if the message can be sent to private chats with bots
    #[must_use]
    pub fn allow_bot_chats(mut self, allow_bot_chats: impl Into<bool>) -> Self {
        self.params.allow_bot_chats = allow_bot_chats.into();
        self
    }

    ///Pass *True* if the message can be sent to group and supergroup chats
    #[must_use]
    pub fn allow_group_chats(mut self, allow_group_chats: impl Into<bool>) -> Self {
        self.params.allow_group_chats = allow_group_chats.into();
        self
    }

    ///Pass *True* if the message can be sent to channel chats
    #[must_use]
    pub fn allow_channel_chats(mut self, allow_channel_chats: impl Into<bool>) -> Self {
        self.params.allow_channel_chats = allow_channel_chats.into();
        self
    }
}

impl Api {
    ///Stores a message that can be sent by a user of a Mini App. Returns a [PreparedInlineMessage](https://core.telegram.org/bots/api/#preparedinlinemessage) object.
    pub fn save_prepared_inline_message(
        &self,
        user_id: impl Into<i64>,
        result: impl Into<InlineQueryResult>,
    ) -> SavePreparedInlineMessageRequest {
        SavePreparedInlineMessageRequest::new(self, user_id, result)
    }
}

// Divider: all content below this line will be preserved after code regen
