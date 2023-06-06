use crate::api::API;
use crate::entities::chat_invite_link::ChatInviteLink;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct RevokeChatInviteLinkParams {
    pub chat_id: ChatId,
    pub invite_link: String,
}

impl_into_future!(RevokeChatInviteLinkRequest<'a>);

///Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
#[derive(Clone)]
pub struct RevokeChatInviteLinkRequest<'a> {
    api: &'a API,
    params: RevokeChatInviteLinkParams,
}

impl<'a> RequestT for RevokeChatInviteLinkRequest<'a> {
    type ParamsType = RevokeChatInviteLinkParams;
    type ReturnType = ChatInviteLink;
    fn get_name() -> &'static str {
        "revokeChatInviteLink"
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
impl<'a> RevokeChatInviteLinkRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, invite_link: String) -> Self {
        Self {
            api,
            params: RevokeChatInviteLinkParams {
                chat_id,
                invite_link,
            },
        }
    }

    ///Unique identifier of the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///The invite link to revoke
    pub fn invite_link(mut self, invite_link: impl Into<String>) -> Self {
        self.params.invite_link = invite_link.into();
        self
    }
}

impl<'a> API {
    ///Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    pub fn revoke_chat_invite_link(
        &'a self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> RevokeChatInviteLinkRequest {
        RevokeChatInviteLinkRequest::new(self, chat_id.into(), invite_link.into())
    }
}

// Divider: all content below this line will be preserved after code regen
