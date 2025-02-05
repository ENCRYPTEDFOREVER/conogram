


use serde::Serialize;

use crate::{
    api::Api, entities::sticker::Sticker,  impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]

pub struct GetForumTopicIconStickersParams {}

impl_into_future!(GetForumTopicIconStickersRequest<'a>);

///Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
#[derive(Clone)]
pub struct GetForumTopicIconStickersRequest<'a> {
    api: &'a Api,
    params: GetForumTopicIconStickersParams,
}

impl RequestT for GetForumTopicIconStickersRequest<'_> {
    type ParamsType = GetForumTopicIconStickersParams;
    type ReturnType = Vec<Sticker>;
    fn get_name() -> &'static str {
        "getForumTopicIconStickers"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> GetForumTopicIconStickersRequest<'a> {
    pub const fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: GetForumTopicIconStickersParams {},
        }
    }
}

impl Api {
    ///Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of [Sticker](https://core.telegram.org/bots/api/#sticker) objects.
    pub const fn get_forum_topic_icon_stickers(&self) -> GetForumTopicIconStickersRequest {
        GetForumTopicIconStickersRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
