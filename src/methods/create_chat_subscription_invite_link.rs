use crate::api::API;
use crate::entities::chat_invite_link::ChatInviteLink;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct CreateChatSubscriptionInviteLinkParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub subscription_period: i64,
    pub subscription_price: i64,
}

impl_into_future!(CreateChatSubscriptionInviteLinkRequest<'a>);

///Use this method to create a [subscription invite link](https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions) for a channel chat. The bot must have the *can\_invite\_users* administrator rights. The link can be edited using the method [editChatSubscriptionInviteLink](https://core.telegram.org/bots/api/#editchatsubscriptioninvitelink) or revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
#[derive(Clone)]
pub struct CreateChatSubscriptionInviteLinkRequest<'a> {
    api: &'a API,
    params: CreateChatSubscriptionInviteLinkParams,
}

impl<'a> RequestT for CreateChatSubscriptionInviteLinkRequest<'a> {
    type ParamsType = CreateChatSubscriptionInviteLinkParams;
    type ReturnType = ChatInviteLink;
    fn get_name() -> &'static str {
        "createChatSubscriptionInviteLink"
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
impl<'a> CreateChatSubscriptionInviteLinkRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: impl Into<ChatId>,
        subscription_period: impl Into<i64>,
        subscription_price: impl Into<i64>,
    ) -> Self {
        Self {
            api,
            params: CreateChatSubscriptionInviteLinkParams {
                chat_id: chat_id.into(),
                subscription_period: subscription_period.into(),
                subscription_price: subscription_price.into(),
                name: Option::default(),
            },
        }
    }

    ///Unique identifier for the target channel chat or username of the target channel (in the format `@channelusername`)
    #[must_use]
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Invite link name; 0-32 characters
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.params.name = Some(name.into());
        self
    }

    ///The number of seconds the subscription will be active for before the next payment. Currently, it must always be 2592000 (30 days).
    #[must_use]
    pub fn subscription_period(mut self, subscription_period: impl Into<i64>) -> Self {
        self.params.subscription_period = subscription_period.into();
        self
    }

    ///The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat; 1-2500
    #[must_use]
    pub fn subscription_price(mut self, subscription_price: impl Into<i64>) -> Self {
        self.params.subscription_price = subscription_price.into();
        self
    }
}

impl<'a> API {
    ///Use this method to create a [subscription invite link](https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions) for a channel chat. The bot must have the *can\_invite\_users* administrator rights. The link can be edited using the method [editChatSubscriptionInviteLink](https://core.telegram.org/bots/api/#editchatsubscriptioninvitelink) or revoked using the method [revokeChatInviteLink](https://core.telegram.org/bots/api/#revokechatinvitelink). Returns the new invite link as a [ChatInviteLink](https://core.telegram.org/bots/api/#chatinvitelink) object.
    pub fn create_chat_subscription_invite_link(
        &'a self,
        chat_id: impl Into<ChatId>,
        subscription_period: impl Into<i64>,
        subscription_price: impl Into<i64>,
    ) -> CreateChatSubscriptionInviteLinkRequest {
        CreateChatSubscriptionInviteLinkRequest::new(
            self,
            chat_id,
            subscription_period,
            subscription_price,
        )
    }
}

// Divider: all content below this line will be preserved after code regen
