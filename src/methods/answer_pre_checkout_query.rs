use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{api::Api, errors::ConogramError, impl_into_future, request::RequestT};

#[derive(Debug, Clone, Serialize)]
pub struct AnswerPreCheckoutQueryParams {
    pub pre_checkout_query_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl_into_future!(AnswerPreCheckoutQueryRequest<'a>);

///Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an [Update](https://core.telegram.org/bots/api/#update) with the field *pre\_checkout\_query*. Use this method to respond to such pre-checkout queries. On success, *True* is returned. **Note:** The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
#[derive(Clone)]
pub struct AnswerPreCheckoutQueryRequest<'a> {
    api: &'a Api,
    params: AnswerPreCheckoutQueryParams,
}

impl RequestT for AnswerPreCheckoutQueryRequest<'_> {
    type ParamsType = AnswerPreCheckoutQueryParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "answerPreCheckoutQuery"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        false
    }
}
impl<'a> AnswerPreCheckoutQueryRequest<'a> {
    pub fn new(
        api: &'a Api,
        pre_checkout_query_id: impl Into<String>,
        ok: impl Into<bool>,
    ) -> Self {
        Self {
            api,
            params: AnswerPreCheckoutQueryParams {
                pre_checkout_query_id: pre_checkout_query_id.into(),
                ok: ok.into(),
                error_message: Option::default(),
            },
        }
    }

    ///Unique identifier for the query to be answered
    #[must_use]
    pub fn pre_checkout_query_id(mut self, pre_checkout_query_id: impl Into<String>) -> Self {
        self.params.pre_checkout_query_id = pre_checkout_query_id.into();
        self
    }

    ///Specify *True* if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use *False* if there are any problems.
    #[must_use]
    pub fn ok(mut self, ok: impl Into<bool>) -> Self {
        self.params.ok = ok.into();
        self
    }

    ///Required if *ok* is *False*. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.
    #[must_use]
    pub fn error_message(mut self, error_message: impl Into<String>) -> Self {
        self.params.error_message = Some(error_message.into());
        self
    }
}

impl Api {
    ///Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an [Update](https://core.telegram.org/bots/api/#update) with the field *pre\_checkout\_query*. Use this method to respond to such pre-checkout queries. On success, *True* is returned. **Note:** The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
    pub fn answer_pre_checkout_query(
        &self,
        pre_checkout_query_id: impl Into<String>,
        ok: impl Into<bool>,
    ) -> AnswerPreCheckoutQueryRequest {
        AnswerPreCheckoutQueryRequest::new(self, pre_checkout_query_id, ok)
    }
}

// Divider: all content below this line will be preserved after code regen
