


use serde::Serialize;

use crate::{api::Api,  impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]

pub struct RefundStarPaymentParams {
    pub user_id: i64,
    pub telegram_payment_charge_id: String,
}

impl_into_future!(RefundStarPaymentRequest<'a>);

///Refunds a successful payment in [Telegram Stars](https://t.me/BotNews/90). Returns *True* on success.
#[derive(Clone)]
pub struct RefundStarPaymentRequest<'a> {
    api: &'a Api,
    params: RefundStarPaymentParams,
}

impl RequestT for RefundStarPaymentRequest<'_> {
    type ParamsType = RefundStarPaymentParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "refundStarPayment"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> RefundStarPaymentRequest<'a> {
    pub fn new(
        api: &'a Api,
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

impl Api {
    ///Refunds a successful payment in [Telegram Stars](https://t.me/BotNews/90). Returns *True* on success.
    pub fn refund_star_payment(
        &self,
        user_id: impl Into<i64>,
        telegram_payment_charge_id: impl Into<String>,
    ) -> RefundStarPaymentRequest {
        RefundStarPaymentRequest::new(self, user_id, telegram_payment_charge_id)
    }
}

// Divider: all content below this line will be preserved after code regen
