use crate::api::API;
use crate::entities::labeled_price::LabeledPrice;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct CreateInvoiceLinkParams {
    pub title: String,
    pub description: String,
    pub payload: String,
    pub provider_token: String,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggested_tip_amounts: Vec<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub need_name: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub need_phone_number: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub need_email: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub need_shipping_address: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub send_phone_number_to_provider: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub send_email_to_provider: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_flexible: bool,
}

impl_into_future!(CreateInvoiceLinkRequest<'a>);

///Use this method to create a link for an invoice. Returns the created invoice link as *String* on success.
#[derive(Clone)]
pub struct CreateInvoiceLinkRequest<'a> {
    api: &'a API,
    params: CreateInvoiceLinkParams,
}

impl<'a> RequestT for CreateInvoiceLinkRequest<'a> {
    type ParamsType = CreateInvoiceLinkParams;
    type ReturnType = String;
    fn get_name() -> &'static str {
        "createInvoiceLink"
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
impl<'a> CreateInvoiceLinkRequest<'a> {
    pub fn new(
        api: &'a API,
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            api,
            params: CreateInvoiceLinkParams {
                title,
                description,
                payload,
                provider_token,
                currency,
                prices,
                max_tip_amount: Option::default(),
                suggested_tip_amounts: Vec::default(),
                provider_data: Option::default(),
                photo_url: Option::default(),
                photo_size: Option::default(),
                photo_width: Option::default(),
                photo_height: Option::default(),
                need_name: bool::default(),
                need_phone_number: bool::default(),
                need_email: bool::default(),
                need_shipping_address: bool::default(),
                send_phone_number_to_provider: bool::default(),
                send_email_to_provider: bool::default(),
                is_flexible: bool::default(),
            },
        }
    }

    ///Product name, 1-32 characters
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.params.title = title.into();
        self
    }

    ///Product description, 1-255 characters
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.params.description = description.into();
        self
    }

    ///Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub fn payload(mut self, payload: impl Into<String>) -> Self {
        self.params.payload = payload.into();
        self
    }

    ///Payment provider token, obtained via [BotFather](https://t.me/botfather)
    pub fn provider_token(mut self, provider_token: impl Into<String>) -> Self {
        self.params.provider_token = provider_token.into();
        self
    }

    ///Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/payments#supported-currencies)
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.params.currency = currency.into();
        self
    }

    ///Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub fn prices(mut self, prices: impl Into<Vec<LabeledPrice>>) -> Self {
        self.params.prices = prices.into();
        self
    }

    ///The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
    pub fn max_tip_amount(mut self, max_tip_amount: impl Into<i64>) -> Self {
        self.params.max_tip_amount = Some(max_tip_amount.into());
        self
    }

    ///A JSON-serialized array of suggested amounts of tips in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\_tip\_amount*.
    pub fn suggested_tip_amounts(mut self, suggested_tip_amounts: impl Into<Vec<i64>>) -> Self {
        self.params.suggested_tip_amounts = suggested_tip_amounts.into();
        self
    }

    ///JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    pub fn provider_data(mut self, provider_data: impl Into<String>) -> Self {
        self.params.provider_data = Some(provider_data.into());
        self
    }

    ///URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    pub fn photo_url(mut self, photo_url: impl Into<String>) -> Self {
        self.params.photo_url = Some(photo_url.into());
        self
    }

    ///Photo size in bytes
    pub fn photo_size(mut self, photo_size: impl Into<i64>) -> Self {
        self.params.photo_size = Some(photo_size.into());
        self
    }

    ///Photo width
    pub fn photo_width(mut self, photo_width: impl Into<i64>) -> Self {
        self.params.photo_width = Some(photo_width.into());
        self
    }

    ///Photo height
    pub fn photo_height(mut self, photo_height: impl Into<i64>) -> Self {
        self.params.photo_height = Some(photo_height.into());
        self
    }

    ///Pass *True* if you require the user's full name to complete the order
    pub fn need_name(mut self, need_name: impl Into<bool>) -> Self {
        self.params.need_name = need_name.into();
        self
    }

    ///Pass *True* if you require the user's phone number to complete the order
    pub fn need_phone_number(mut self, need_phone_number: impl Into<bool>) -> Self {
        self.params.need_phone_number = need_phone_number.into();
        self
    }

    ///Pass *True* if you require the user's email address to complete the order
    pub fn need_email(mut self, need_email: impl Into<bool>) -> Self {
        self.params.need_email = need_email.into();
        self
    }

    ///Pass *True* if you require the user's shipping address to complete the order
    pub fn need_shipping_address(mut self, need_shipping_address: impl Into<bool>) -> Self {
        self.params.need_shipping_address = need_shipping_address.into();
        self
    }

    ///Pass *True* if the user's phone number should be sent to the provider
    pub fn send_phone_number_to_provider(
        mut self,
        send_phone_number_to_provider: impl Into<bool>,
    ) -> Self {
        self.params.send_phone_number_to_provider = send_phone_number_to_provider.into();
        self
    }

    ///Pass *True* if the user's email address should be sent to the provider
    pub fn send_email_to_provider(mut self, send_email_to_provider: impl Into<bool>) -> Self {
        self.params.send_email_to_provider = send_email_to_provider.into();
        self
    }

    ///Pass *True* if the final price depends on the shipping method
    pub fn is_flexible(mut self, is_flexible: impl Into<bool>) -> Self {
        self.params.is_flexible = is_flexible.into();
        self
    }
}

impl<'a> API {
    ///Use this method to create a link for an invoice. Returns the created invoice link as *String* on success.
    pub fn create_invoice_link(
        &'a self,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        provider_token: impl Into<String>,
        currency: impl Into<String>,
        prices: impl Into<Vec<LabeledPrice>>,
    ) -> CreateInvoiceLinkRequest {
        CreateInvoiceLinkRequest::new(
            self,
            title.into(),
            description.into(),
            payload.into(),
            provider_token.into(),
            currency.into(),
            prices.into(),
        )
    }
}

// Divider: all content below this line will be preserved after code regen
