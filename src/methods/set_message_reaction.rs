use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::entities::reaction_type::ReactionType;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetMessageReactionParams {
    pub chat_id: ChatId,
    pub message_id: i64,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reaction: Vec<ReactionType>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_big: bool,
}

impl_into_future!(SetMessageReactionRequest<'a>);

///Use this method to change the chosen reactions on a message. Service messages can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Returns *True* on success.
#[derive(Clone)]
pub struct SetMessageReactionRequest<'a> {
    api: &'a API,
    params: SetMessageReactionParams,
}

impl<'a> RequestT for SetMessageReactionRequest<'a> {
    type ParamsType = SetMessageReactionParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setMessageReaction"
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
impl<'a> SetMessageReactionRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, message_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: SetMessageReactionParams {
                chat_id: chat_id.into(),
                message_id: message_id.into(),
                reaction: Vec::default(),
                is_big: bool::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Identifier of the target message. If the message belongs to a media group, the reaction is set to the first non-deleted message in the group instead.
    pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
        self.params.message_id = message_id.into();
        self
    }

    ///New list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators.
    pub fn reaction(mut self, reaction: impl IntoIterator<Item = impl Into<ReactionType>>) -> Self {
        self.params.reaction = reaction.into_iter().map(Into::into).collect();
        self
    }

    ///Pass *True* to set the reaction with a big animation
    pub fn is_big(mut self, is_big: impl Into<bool>) -> Self {
        self.params.is_big = is_big.into();
        self
    }
}

impl<'a> API {
    ///Use this method to change the chosen reactions on a message. Service messages can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. Returns *True* on success.
    pub fn set_message_reaction(
        &'a self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<i64>,
    ) -> SetMessageReactionRequest {
        SetMessageReactionRequest::new(self, chat_id.into(), message_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
