use crate::api::API;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct EditForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl_into_future!(EditForumTopicRequest<'a>);

///Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
#[derive(Clone)]
pub struct EditForumTopicRequest<'a> {
    api: &'a API,
    params: EditForumTopicParams,
}

impl<'a> RequestT for EditForumTopicRequest<'a> {
    type ParamsType = EditForumTopicParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "editForumTopic"
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
impl<'a> EditForumTopicRequest<'a> {
    pub fn new(api: &'a API, chat_id: ChatId, message_thread_id: i64) -> Self {
        Self {
            api,
            params: EditForumTopicParams {
                chat_id,
                message_thread_id,
                name: Option::default(),
                icon_custom_emoji_id: Option::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread of the forum topic
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = message_thread_id.into();
        self
    }

    ///New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    ///New unique identifier of the custom emoji shown as the topic icon. Use [getForumTopicIconStickers](https://core.telegram.org/bots/api/#getforumtopiciconstickers) to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept
    pub fn icon_custom_emoji_id(mut self, icon_custom_emoji_id: impl Into<String>) -> Self {
        self.params.icon_custom_emoji_id = Some(icon_custom_emoji_id.into());
        self
    }
}

impl<'a> API {
    ///Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have *can\_manage\_topics* administrator rights, unless it is the creator of the topic. Returns *True* on success.
    pub fn edit_forum_topic(
        &'a self,
        chat_id: impl Into<ChatId>,
        message_thread_id: impl Into<i64>,
    ) -> EditForumTopicRequest {
        EditForumTopicRequest::new(self, chat_id.into(), message_thread_id.into())
    }
}

// Divider: all content below this line will be preserved after code regen
