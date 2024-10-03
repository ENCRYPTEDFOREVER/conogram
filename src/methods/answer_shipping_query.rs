use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API, entities::shipping_option::ShippingOption, errors::ConogramError, impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct AnswerShippingQueryParams {
    pub shipping_query_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shipping_options: Vec<ShippingOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl_into_future!(AnswerShippingQueryRequest<'a>);

///If you sent an invoice requesting a shipping address and the parameter *is\_flexible* was specified, the Bot API will send an [Update](https://core.telegram.org/bots/api/#update) with a *shipping\_query* field to the bot. Use this method to reply to shipping queries. On success, *True* is returned.
#[derive(Clone)]
pub struct AnswerShippingQueryRequest<'a> {
    api: &'a API,
    params: AnswerShippingQueryParams,
}

impl<'a> RequestT for AnswerShippingQueryRequest<'a> {
    type ParamsType = AnswerShippingQueryParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "answerShippingQuery"
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
impl<'a> AnswerShippingQueryRequest<'a> {
    pub fn new(api: &'a API, shipping_query_id: impl Into<String>, ok: impl Into<bool>) -> Self {
        Self {
            api,
            params: AnswerShippingQueryParams {
                shipping_query_id: shipping_query_id.into(),
                ok: ok.into(),
                shipping_options: Vec::default(),
                error_message: Option::default(),
            },
        }
    }

    ///Unique identifier for the query to be answered
    #[must_use]
    pub fn shipping_query_id(mut self, shipping_query_id: impl Into<String>) -> Self {
        self.params.shipping_query_id = shipping_query_id.into();
        self
    }

    ///Pass *True* if delivery to the specified address is possible and *False* if there are any problems (for example, if delivery to the specified address is not possible)
    #[must_use]
    pub fn ok(mut self, ok: impl Into<bool>) -> Self {
        self.params.ok = ok.into();
        self
    }

    ///Required if *ok* is *True*. A JSON-serialized array of available shipping options.
    #[must_use]
    pub fn shipping_options(
        mut self,
        shipping_options: impl IntoIterator<Item = impl Into<ShippingOption>>,
    ) -> Self {
        self.params.shipping_options = shipping_options.into_iter().map(Into::into).collect();
        self
    }

    ///Required if *ok* is *False*. Error message in human readable form that explains why it is impossible to complete the order (e.g. "Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user.
    #[must_use]
    pub fn error_message(mut self, error_message: impl Into<String>) -> Self {
        self.params.error_message = Some(error_message.into());
        self
    }
}

impl API {
    ///If you sent an invoice requesting a shipping address and the parameter *is\_flexible* was specified, the Bot API will send an [Update](https://core.telegram.org/bots/api/#update) with a *shipping\_query* field to the bot. Use this method to reply to shipping queries. On success, *True* is returned.
    pub fn answer_shipping_query(
        &self,
        shipping_query_id: impl Into<String>,
        ok: impl Into<bool>,
    ) -> AnswerShippingQueryRequest {
        AnswerShippingQueryRequest::new(self, shipping_query_id, ok)
    }
}

// Divider: all content below this line will be preserved after code regen
