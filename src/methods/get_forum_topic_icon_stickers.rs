use crate::api::API;
use crate::entities::sticker::Sticker;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

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
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: GetForumTopicIconStickersParams {},
        }
    }
}

impl<'a> API {
    ///Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    pub fn get_forum_topic_icon_stickers(&'a self) -> GetForumTopicIconStickersRequest {
        GetForumTopicIconStickersRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
