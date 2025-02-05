


use serde::Serialize;

use crate::{
    api::Api, entities::misc::chat_id::ChatId,  impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]

pub struct DeleteMessageParams {
    pub chat_id: ChatId,
    pub message_id: i64,
}

impl_into_future!(DeleteMessageRequest<'a>);

///Use this method to delete a message, including service messages, with the following limitations:  
///\- A message can only be deleted if it was sent less than 48 hours ago.  
///\- Service messages about a supergroup, channel, or forum topic creation can't be deleted.  
///\- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.  
///\- Bots can delete outgoing messages in private chats, groups, and supergroups.  
///\- Bots can delete incoming messages in private chats.  
///\- Bots granted *can\_post\_messages* permissions can delete outgoing messages in channels.  
///\- If the bot is an administrator of a group, it can delete any message there.  
///\- If the bot has *can\_delete\_messages* permission in a supergroup or a channel, it can delete any message there.  
///Returns *True* on success.
#[derive(Clone)]
pub struct DeleteMessageRequest<'a> {
    api: &'a Api,
    params: DeleteMessageParams,
}

impl RequestT for DeleteMessageRequest<'_> {
    type ParamsType = DeleteMessageParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "deleteMessage"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> DeleteMessageRequest<'a> {
    pub fn new(api: &'a Api, chat_id: impl Into<ChatId>, message_id: impl Into<i64>) -> Self {
        Self {
            api,
            params: DeleteMessageParams {
                chat_id: chat_id.into(),
                message_id: message_id.into(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Identifier of the message to delete
    #[must_use]
    pub fn message_id(mut self, message_id: impl Into<i64>) -> Self {
        self.params.message_id = message_id.into();
        self
    }
}

impl Api {
    ///Use this method to delete a message, including service messages, with the following limitations:  
    ///\- A message can only be deleted if it was sent less than 48 hours ago.  
    ///\- Service messages about a supergroup, channel, or forum topic creation can't be deleted.  
    ///\- A dice message in a private chat can only be deleted if it was sent more than 24 hours ago.  
    ///\- Bots can delete outgoing messages in private chats, groups, and supergroups.  
    ///\- Bots can delete incoming messages in private chats.  
    ///\- Bots granted *can\_post\_messages* permissions can delete outgoing messages in channels.  
    ///\- If the bot is an administrator of a group, it can delete any message there.  
    ///\- If the bot has *can\_delete\_messages* permission in a supergroup or a channel, it can delete any message there.  
    ///Returns *True* on success.
    pub fn delete_message(
        &self,
        chat_id: impl Into<ChatId>,
        message_id: impl Into<i64>,
    ) -> DeleteMessageRequest {
        DeleteMessageRequest::new(self, chat_id, message_id)
    }
}

// Divider: all content below this line will be preserved after code regen
