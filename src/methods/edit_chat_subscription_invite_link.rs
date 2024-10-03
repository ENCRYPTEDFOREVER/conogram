use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{chat_invite_link::ChatInviteLink, misc::chat_id::ChatId},
    errors::ConogramError,
    impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct EditChatSubscriptionInviteLinkParams {
    pub chat_id: ChatId,
    pub invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl_into_future!(EditChatSubscriptionInviteLinkRequest<'a>);

///Use this method to edit a subscription invite link created by the bot. The bot must have the *can\_invite\_users* administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
#[derive(Clone)]
pub struct EditChatSubscriptionInviteLinkRequest<'a> {
    api: &'a API,
    params: EditChatSubscriptionInviteLinkParams,
}

impl<'a> RequestT for EditChatSubscriptionInviteLinkRequest<'a> {
    type ParamsType = EditChatSubscriptionInviteLinkParams;
    type ReturnType = ChatInviteLink;
    fn get_name() -> &'static str {
        "editChatSubscriptionInviteLink"
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
impl<'a> EditChatSubscriptionInviteLinkRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, invite_link: impl Into<String>) -> Self {
        Self {
            api,
            params: EditChatSubscriptionInviteLinkParams {
                chat_id: chat_id.into(),
                invite_link: invite_link.into(),
                name: Option::default(),
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
}

impl API {
    ///Use this method to edit a subscription invite link created by the bot. The bot must have the *can\_invite\_users* administrator rights. Returns the edited invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    pub fn edit_chat_subscription_invite_link(
        &self,
        chat_id: impl Into<ChatId>,
        invite_link: impl Into<String>,
    ) -> EditChatSubscriptionInviteLinkRequest {
        EditChatSubscriptionInviteLinkRequest::new(self, chat_id, invite_link)
    }
}

// Divider: all content below this line will be preserved after code regen
