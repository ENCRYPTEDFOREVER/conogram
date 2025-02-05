


use serde::Serialize;

use crate::{
    api::Api, entities::labeled_price::LabeledPrice,  impl_into_future,
    request::RequestT, utils::deserialize_utils::is_false,
};

#[derive(Debug, Clone, Serialize)]

pub struct CreateInvoiceLinkParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub title: String,
    pub description: String,
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,
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
    api: &'a Api,
    params: CreateInvoiceLinkParams,
}

impl RequestT for CreateInvoiceLinkRequest<'_> {
    type ParamsType = CreateInvoiceLinkParams;
    type ReturnType = String;
    fn get_name() -> &'static str {
        "createInvoiceLink"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
}
impl<'a> CreateInvoiceLinkRequest<'a> {
    pub fn new(
        api: &'a Api,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: impl IntoIterator<Item = impl Into<LabeledPrice>>,
    ) -> Self {
        Self {
            api,
            params: CreateInvoiceLinkParams {
                title: title.into(),
                description: description.into(),
                payload: payload.into(),
                currency: currency.into(),
                prices: prices.into_iter().map(Into::into).collect(),
                business_connection_id: Option::default(),
                provider_token: Option::default(),
                subscription_period: Option::default(),
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

    ///Unique identifier of the business connection on behalf of which the link will be created
    #[must_use]
    pub fn business_connection_id(mut self, business_connection_id: impl Into<String>) -> Self {
        self.params.business_connection_id = Some(business_connection_id.into());
        self
    }

    ///Product name, 1-32 characters
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.params.title = title.into();
        self
    }

    ///Product description, 1-255 characters
    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.params.description = description.into();
        self
    }

    ///Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    #[must_use]
    pub fn payload(mut self, payload: impl Into<String>) -> Self {
        self.params.payload = payload.into();
        self
    }

    ///Payment provider token, obtained via [@BotFather](https://t.me/botfather). Pass an empty string for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[must_use]
    pub fn provider_token(mut self, provider_token: impl Into<String>) -> Self {
        self.params.provider_token = Some(provider_token.into());
        self
    }

    ///Three-letter ISO 4217 currency code, see [more on currencies](https://core.telegram.org/bots/payments#supported-currencies). Pass “XTR” for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[must_use]
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.params.currency = currency.into();
        self
    }

    ///Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[must_use]
    pub fn prices(mut self, prices: impl IntoIterator<Item = impl Into<LabeledPrice>>) -> Self {
        self.params.prices = prices.into_iter().map(Into::into).collect();
        self
    }

    ///The number of seconds the subscription will be active for before the next payment. The currency must be set to “XTR” (Telegram Stars) if the parameter is used. Currently, it must always be 2592000 (30 days) if specified.
    #[must_use]
    pub fn subscription_period(mut self, subscription_period: impl Into<i64>) -> Self {
        self.params.subscription_period = Some(subscription_period.into());
        self
    }

    ///The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [currencies.json](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[must_use]
    pub fn max_tip_amount(mut self, max_tip_amount: impl Into<i64>) -> Self {
        self.params.max_tip_amount = Some(max_tip_amount.into());
        self
    }

    ///A JSON-serialized array of suggested amounts of tips in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max\_tip\_amount*.
    #[must_use]
    pub fn suggested_tip_amounts(
        mut self,
        suggested_tip_amounts: impl IntoIterator<Item = impl Into<i64>>,
    ) -> Self {
        self.params.suggested_tip_amounts =
            suggested_tip_amounts.into_iter().map(Into::into).collect();
        self
    }

    ///JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[must_use]
    pub fn provider_data(mut self, provider_data: impl Into<String>) -> Self {
        self.params.provider_data = Some(provider_data.into());
        self
    }

    ///URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[must_use]
    pub fn photo_url(mut self, photo_url: impl Into<String>) -> Self {
        self.params.photo_url = Some(photo_url.into());
        self
    }

    ///Photo size in bytes
    #[must_use]
    pub fn photo_size(mut self, photo_size: impl Into<i64>) -> Self {
        self.params.photo_size = Some(photo_size.into());
        self
    }

    ///Photo width
    #[must_use]
    pub fn photo_width(mut self, photo_width: impl Into<i64>) -> Self {
        self.params.photo_width = Some(photo_width.into());
        self
    }

    ///Photo height
    #[must_use]
    pub fn photo_height(mut self, photo_height: impl Into<i64>) -> Self {
        self.params.photo_height = Some(photo_height.into());
        self
    }

    ///Pass *True* if you require the user's full name to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[must_use]
    pub fn need_name(mut self, need_name: impl Into<bool>) -> Self {
        self.params.need_name = need_name.into();
        self
    }

    ///Pass *True* if you require the user's phone number to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[must_use]
    pub fn need_phone_number(mut self, need_phone_number: impl Into<bool>) -> Self {
        self.params.need_phone_number = need_phone_number.into();
        self
    }

    ///Pass *True* if you require the user's email address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[must_use]
    pub fn need_email(mut self, need_email: impl Into<bool>) -> Self {
        self.params.need_email = need_email.into();
        self
    }

    ///Pass *True* if you require the user's shipping address to complete the order. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[must_use]
    pub fn need_shipping_address(mut self, need_shipping_address: impl Into<bool>) -> Self {
        self.params.need_shipping_address = need_shipping_address.into();
        self
    }

    ///Pass *True* if the user's phone number should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[must_use]
    pub fn send_phone_number_to_provider(
        mut self,
        send_phone_number_to_provider: impl Into<bool>,
    ) -> Self {
        self.params.send_phone_number_to_provider = send_phone_number_to_provider.into();
        self
    }

    ///Pass *True* if the user's email address should be sent to the provider. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[must_use]
    pub fn send_email_to_provider(mut self, send_email_to_provider: impl Into<bool>) -> Self {
        self.params.send_email_to_provider = send_email_to_provider.into();
        self
    }

    ///Pass *True* if the final price depends on the shipping method. Ignored for payments in [Telegram Stars](https://t.me/BotNews/90).
    #[must_use]
    pub fn is_flexible(mut self, is_flexible: impl Into<bool>) -> Self {
        self.params.is_flexible = is_flexible.into();
        self
    }
}

impl Api {
    ///Use this method to create a link for an invoice. Returns the created invoice link as *String* on success.
    pub fn create_invoice_link(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: impl IntoIterator<Item = impl Into<LabeledPrice>>,
    ) -> CreateInvoiceLinkRequest {
        CreateInvoiceLinkRequest::new(self, title, description, payload, currency, prices)
    }
}

// Divider: all content below this line will be preserved after code regen
