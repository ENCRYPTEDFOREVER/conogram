use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api, entities::misc::chat_id::ChatId, errors::ConogramError, impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct ExportChatInviteLinkParams {
    pub chat_id: ChatId,
}

impl_into_future!(ExportChatInviteLinkRequest<'a>);

///Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as *String* on success.
#[derive(Clone)]
pub struct ExportChatInviteLinkRequest<'a> {
    api: &'a Api,
    params: ExportChatInviteLinkParams,
}

impl RequestT for ExportChatInviteLinkRequest<'_> {
    type ParamsType = ExportChatInviteLinkParams;
    type ReturnType = String;
    fn get_name() -> &'static str {
        "exportChatInviteLink"
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
impl<'a> ExportChatInviteLinkRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>) -> Self {
        Self {
            api,
            params: ExportChatInviteLinkParams {
                chat_id: chat_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }
}

impl Api {
    ///Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as *String* on success.
    pub fn export_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
    ) -> ExportChatInviteLinkRequest {
        ExportChatInviteLinkRequest::new(self, chat_id)
    }
}

// Divider: all content below this line will be preserved after code regen
