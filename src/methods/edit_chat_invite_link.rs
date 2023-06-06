use crate::api::API;
use crate::entities::chat_invite_link::ChatInviteLink;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct EditChatInviteLinkParams {
    pub chat_id: ChatId,
    pub invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub creates_join_request: bool,
}

impl_into_future!(EditChatInviteLinkRequest<'a>);

///Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
#[derive(Clone)]
pub struct EditChatInviteLinkRequest<'a> {
    api: &'a API,
    params: EditChatInviteLinkParams,
}

impl<'a> RequestT for EditChatInviteLinkRequest<'a> {
    type ParamsType = EditChatInviteLinkParams;
    type ReturnType = ChatInviteLink;
    fn get_name() -> &'static str {
        "editChatInviteLink"
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
impl<'a> EditChatInviteLinkRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, invite_link: String) -> Self {
        Self {
            api,
            params: EditChatInviteLinkParams {
                chat_id,
                invite_link,
                name: Option::default(),
                expire_date: Option::default(),
                member_limit: Option::default(),
                creates_join_request: bool::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///The invite link to edit
    pub fn invite_link(mut self, invite_link: impl Into<String>) -> Self {
        self.params.invite_link = invite_link.into();
        self
    }

    ///Invite link name; 0-32 characters
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    ///Point in time (Unix timestamp) when the link will expire
    pub fn expire_date(mut self, expire_date: impl Into<i64>) -> Self {
        self.params.expire_date = Some(expire_date.into());
        self
    }

    ///The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    pub fn member_limit(mut self, member_limit: impl Into<i64>) -> Self {
        self.params.member_limit = Some(member_limit.into());
        self
    }

    ///*True*, if users joining the chat via the link need to be approved by chat administrators. If *True*, *member\_limit* can't be specified
    pub fn creates_join_request(mut self, creates_join_request: impl Into<bool>) -> Self {
        self.params.creates_join_request = creates_join_request.into();
        self
    }
}

impl<'a> API {
    ///Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    pub fn edit_chat_invite_link(
        &'a self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> EditChatInviteLinkRequest {
        EditChatInviteLinkRequest::new(self, chat_id.into(), invite_link.into())
    }
}

// Divider: all content below this line will be preserved after code regen
