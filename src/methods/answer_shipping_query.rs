use crate::api::API;
use crate::entities::shipping_option::ShippingOption;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

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
    pub fn new(api: &'a API, shipping_query_id: String, ok: bool) -> Self {
        Self {
            api,
            params: AnswerShippingQueryParams {
                shipping_query_id,
                ok,
                shipping_options: Vec::default(),
                error_message: Option::default(),
            },
        }
    }

    ///Unique identifier for the query to be answered
    pub fn shipping_query_id(mut self, shipping_query_id: impl Into<String>) -> Self {
        self.params.shipping_query_id = shipping_query_id.into();
        self
    }

    ///Pass *True* if delivery to the specified address is possible and *False* if there are any problems (for example, if delivery to the specified address is not possible)
    pub fn ok(mut self, ok: impl Into<bool>) -> Self {
        self.params.ok = ok.into();
        self
    }

    ///Required if *ok* is *True*. A JSON-serialized array of available shipping options.
    pub fn shipping_options(mut self, shipping_options: impl Into<Vec<ShippingOption>>) -> Self {
        self.params.shipping_options = shipping_options.into();
        self
    }

    ///Required if *ok* is *False*. Error message in human readable form that explains why it is impossible to complete the order (e.g. "Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user.
    pub fn error_message(mut self, error_message: impl Into<String>) -> Self {
        self.params.error_message = Some(error_message.into());
        self
    }
}

impl<'a> API {
    ///If you sent an invoice requesting a shipping address and the parameter *is\_flexible* was specified, the Bot API will send an [Update](https://core.telegram.org/bots/api/#update) with a *shipping\_query* field to the bot. Use this method to reply to shipping queries. On success, *True* is returned.
    pub fn answer_shipping_query(
        &'a self,
        shipping_query_id: impl Into<String>,
        ok: impl Into<bool>,
    ) -> AnswerShippingQueryRequest {
        AnswerShippingQueryRequest::new(self, shipping_query_id.into(), ok.into())
    }
}

// Divider: all content below this line will be preserved after code regen
