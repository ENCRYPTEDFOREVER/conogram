use crate::api::API;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct RefundStarPaymentParams {
    pub user_id: i64,
    pub telegram_payment_charge_id: String,
}

impl_into_future!(RefundStarPaymentRequest<'a>);

///Refunds a successful payment in [Telegram Stars](https://t.me/BotNews/90). Returns *True* on success.
#[derive(Clone)]
pub struct RefundStarPaymentRequest<'a> {
    api: &'a API,
    params: RefundStarPaymentParams,
}

impl<'a> RequestT for RefundStarPaymentRequest<'a> {
    type ParamsType = RefundStarPaymentParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "refundStarPayment"
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
impl<'a> RefundStarPaymentRequest<'a> {
    pub fn new(
        api: &'a API,
        user_id: impl Into<i64>,
        telegram_payment_charge_id: impl Into<String>,
    ) -> Self {
        Self {
            api,
            params: RefundStarPaymentParams {
                user_id: user_id.into(),
                telegram_payment_charge_id: telegram_payment_charge_id.into(),
            },
        }
    }

    ///Identifier of the user whose payment will be refunded
    #[must_use]
    pub fn user_id(mut self, user_id: impl Into<i64>) -> Self {
        self.params.user_id = user_id.into();
        self
    }

    ///Telegram payment identifier
    #[must_use]
    pub fn telegram_payment_charge_id(
        mut self,
        telegram_payment_charge_id: impl Into<String>,
    ) -> Self {
        self.params.telegram_payment_charge_id = telegram_payment_charge_id.into();
        self
    }
}

impl<'a> API {
    ///Refunds a successful payment in [Telegram Stars](https://t.me/BotNews/90). Returns *True* on success.
    pub fn refund_star_payment(
        &'a self,
        user_id: impl Into<i64>,
        telegram_payment_charge_id: impl Into<String>,
    ) -> RefundStarPaymentRequest {
        RefundStarPaymentRequest::new(self, user_id, telegram_payment_charge_id)
    }
}

// Divider: all content below this line will be preserved after code regen
