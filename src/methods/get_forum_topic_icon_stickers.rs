use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API, entities::sticker::Sticker, errors::ConogramError, impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetForumTopicIconStickersParams {}

impl_into_future!(GetForumTopicIconStickersRequest<'a>);

///Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
#[derive(Clone)]
pub struct GetForumTopicIconStickersRequest<'a> {
    api: &'a API,
    params: GetForumTopicIconStickersParams,
}

impl<'a> RequestT for GetForumTopicIconStickersRequest<'a> {
    type ParamsType = GetForumTopicIconStickersParams;
    type ReturnType = Vec<Sticker>;
    fn get_name() -> &'static str {
        "getForumTopicIconStickers"
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
impl<'a> GetForumTopicIconStickersRequest<'a> {
    pub const fn new(api: &'a API) -> Self {
        Self {
            api,
            params: GetForumTopicIconStickersParams {},
        }
    }
}

impl API {
    ///Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    pub const fn get_forum_topic_icon_stickers(&self) -> GetForumTopicIconStickersRequest {
        GetForumTopicIconStickersRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
