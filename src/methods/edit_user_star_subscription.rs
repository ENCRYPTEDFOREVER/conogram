


use serde::Serialize;

use crate::{api::Api,  impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]

pub struct EditUserStarSubscriptionParams {
    pub user_id: i64,
    pub telegram_payment_charge_id: String,
    pub is_canceled: bool,
}

impl_into_future!(EditUserStarSubscriptionRequest<'a>);

///Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns *True* on success.
#[derive(Clone)]
pub struct EditUserStarSubscriptionRequest<'a> {
    api: &'a Api,
    params: EditUserStarSubscriptionParams,
}

impl RequestT for EditUserStarSubscriptionRequest<'_> {
    type ParamsType = EditUserStarSubscriptionParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "editUserStarSubscription"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> EditUserStarSubscriptionRequest<'a> {
    pub fn new(
        api: &'a Api,
        user_id: impl Into<i64>,
        telegram_payment_charge_id: impl Into<String>,
        is_canceled: impl Into<bool>,
    ) -> Self {
        Self {
            api,
            params: EditUserStarSubscriptionParams {
                user_id: user_id.into(),
                telegram_payment_charge_id: telegram_payment_charge_id.into(),
                is_canceled: is_canceled.into(),
            },
        }
    }

    ///Identifier of the user whose subscription will be edited
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Telegram payment identifier for the subscription
    #[must_use]
    pub fn telegram_payment_charge_id(
        mut self,
        telegram_payment_charge_id: impl Into<String>,
    ) -> Self {
        self.params.telegram_payment_charge_id = telegram_payment_charge_id.into();
        self
    }

    ///Pass *True* to cancel extension of the user subscription; the subscription must be active up to the end of the current subscription period. Pass *False* to allow the user to re-enable a subscription that was previously canceled by the bot.
    #[must_use]
    pub fn is_canceled(mut self, is_canceled: impl Into<bool>) -> Self {
        self.params.is_canceled = is_canceled.into();
        self
    }
}

impl Api {
    ///Allows the bot to cancel or re-enable extension of a subscription paid in Telegram Stars. Returns *True* on success.
    pub fn edit_user_star_subscription(
        &self,
        user_id: impl Into<i64>,
        telegram_payment_charge_id: impl Into<String>,
        is_canceled: impl Into<bool>,
    ) -> EditUserStarSubscriptionRequest {
        EditUserStarSubscriptionRequest::new(self, user_id, telegram_payment_charge_id, is_canceled)
    }
}

// Divider: all content below this line will be preserved after code regen
