use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{forum_topic::ForumTopic, misc::chat_id::ChatId},
    errors::ConogramError,
    impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct CreateForumTopicParams {
    pub chat_id: ChatId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<CreateForumTopicIconColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

impl_into_future!(CreateForumTopicRequest<'a>);

///Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. Returns information about the created topic as a [ForumTopic](https://core.telegram.org/bots/api/#forumtopic) object.
#[derive(Clone)]
pub struct CreateForumTopicRequest<'a> {
    api: &'a API,
    params: CreateForumTopicParams,
}

impl<'a> RequestT for CreateForumTopicRequest<'a> {
    type ParamsType = CreateForumTopicParams;
    type ReturnType = ForumTopic;
    fn get_name() -> &'static str {
        "createForumTopic"
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
impl<'a> CreateForumTopicRequest<'a> {
    pub fn new(api: &'a API, chat_id: impl Into<ChatId>, name: impl Into<String>) -> Self {
        Self {
            api,
            params: CreateForumTopicParams {
                chat_id: chat_id.into(),
                name: name.into(),
                icon_color: Option::default(),
                icon_custom_emoji_id: Option::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Topic name, 1-128 characters
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = name.into();
        self
    }

    ///Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)
    #[must_use]
    pub fn icon_color(mut self, icon_color: impl Into<CreateForumTopicIconColor>) -> Self {
        self.params.icon_color = Some(icon_color.into());
        self
    }

    ///Unique identifier of the custom emoji shown as the topic icon. Use [getForumTopicIconStickers](https://core.telegram.org/bots/api/#getforumtopiciconstickers) to get all allowed custom emoji identifiers.
    #[must_use]
    pub fn icon_custom_emoji_id(mut self, icon_custom_emoji_id: impl Into<String>) -> Self {
        self.params.icon_custom_emoji_id = Some(icon_custom_emoji_id.into());
        self
    }
}

impl API {
    ///Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the *can\_manage\_topics* administrator rights. Returns information about the created topic as a [ForumTopic](https://core.telegram.org/bots/api/#forumtopic) object.
    pub fn create_forum_topic(
        &self,
        chat_id: impl Into<ChatId>,
        name: impl Into<String>,
    ) -> CreateForumTopicRequest {
        CreateForumTopicRequest::new(self, chat_id, name)
    }
}

///Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "icon_color")]
pub enum CreateForumTopicIconColor {
    #[default]
    _6FB9F0 = 0x6fb9f0,
    _FFD67E = 0xffd67e,
    _CB86DB = 0xcb86db,
    _8EEE98 = 0x8eee98,
    _FF93B2 = 0xff93b2,
    _FB6F5F = 0xfb6f5f,
}

// Divider: all content below this line will be preserved after code regen
