


use serde::Serialize;

use crate::{
    api::Api,
    entities::{chat_invite_link::ChatInviteLink, misc::chat_id::ChatId},
    
    impl_into_future,
    request::RequestT,
    utils::deserialize_utils::is_false,
};

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
    #[serde(default, skip_serializing_if = "is_false")]
    pub creates_join_request: bool,
}

impl_into_future!(EditChatInviteLinkRequest<'a>);

///Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
#[derive(Clone)]
pub struct EditChatInviteLinkRequest<'a> {
    api: &'a Api,
    params: EditChatInviteLinkParams,
}

impl RequestT for EditChatInviteLinkRequest<'_> {
    type ParamsType = EditChatInviteLinkParams;
    type ReturnType = ChatInviteLink;
    fn get_name() -> &'static str {
        "editChatInviteLink"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> EditChatInviteLinkRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>, invite_link: impl Into<String>) -> Self {
        Self {
            api,
            params: EditChatInviteLinkParams {
                chat_id: chat_id.into(),
                invite_link: invite_link.into(),
                name: Option::default(),
                expire_date: Option::default(),
                member_limit: Option::default(),
                creates_join_request: bool::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///The invite link to edit
    #[must_use]
    pub fn invite_link(mut self, invite_link: impl Into<String>) -> Self {
        self.params.invite_link = invite_link.into();
        self
    }

    ///Invite link name; 0-32 characters
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    ///Point in time (Unix timestamp) when the link will expire
    #[must_use]
    pub fn expire_date(mut self, expire_date: impl Into<i64>) -> Self {
        self.params.expire_date = Some(expire_date.into());
        self
    }

    ///The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[must_use]
    pub fn member_limit(mut self, member_limit: impl Into<i64>) -> Self {
        self.params.member_limit = Some(member_limit.into());
        self
    }

    ///*True*, if users joining the chat via the link need to be approved by chat administrators. If *True*, *member\_limit* can't be specified
    #[must_use]
    pub fn creates_join_request(mut self, creates_join_request: impl Into<bool>) -> Self {
        self.params.creates_join_request = creates_join_request.into();
        self
    }
}

impl Api {
    ///Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    pub fn edit_chat_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> EditChatInviteLinkRequest {
        EditChatInviteLinkRequest::new(self, chat_id, invite_link)
    }
}

// Divider: all content below this line will be preserved after code regen
