use crate::api::API;
use crate::entities::inline_keyboard_markup::InlineKeyboardMarkup;
use crate::entities::labeled_price::LabeledPrice;
use crate::entities::message::Message;
use crate::entities::misc::chat_id::ChatId;
use crate::errors::Error;
use crate::impl_into_future;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SendInvoiceParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
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
    pub start_parameter: Option<String>,
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
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_notification: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub protect_content: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl_into_future!(SendInvoiceRequest<'a>);

///Use this method to send invoices. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
#[derive(Clone)]
pub struct SendInvoiceRequest<'a> {
    api: &'a API,
    params: SendInvoiceParams,
}

impl<'a> RequestT for SendInvoiceRequest<'a> {
    type ParamsType = SendInvoiceParams;
    type ReturnType = Message;
    fn get_name() -> &'static str {
        "sendInvoice"
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
impl<'a> SendInvoiceRequest<'a> {
    pub fn new(
        api: &'a API,
        chat_id: ChatId,
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            api,
            params: SendInvoiceParams {
                chat_id,
                title,
                description,
                payload,
                provider_token,
                currency,
                prices,
                message_thread_id: Option::default(),
                max_tip_amount: Option::default(),
                suggested_tip_amounts: Vec::default(),
                start_parameter: Option::default(),
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
                disable_notification: bool::default(),
                protect_content: bool::default(),
                reply_to_message_id: Option::default(),
                allow_sending_without_reply: bool::default(),
                reply_markup: Option::default(),
            },
        }
    }

    ///Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub fn chat_id(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.params.chat_id = chat_id.into();
        self
    }

    ///Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub fn message_thread_id(mut self, message_thread_id: impl Into<i64>) -> Self {
        self.params.message_thread_id = Some(message_thread_id.into());
        self
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

    ///Payment provider token, obtained via [@BotFather](https://t.me/botfather)
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

    ///Unique deep-linking parameter. If left empty, **forwarded copies** of the sent message will have a *Pay* button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a *URL* button with a deep link to the bot (instead of a *Pay* button), with the value used as the start parameter
    pub fn start_parameter(mut self, start_parameter: impl Into<String>) -> Self {
        self.params.start_parameter = Some(start_parameter.into());
        self
    }

    ///JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    pub fn provider_data(mut self, provider_data: impl Into<String>) -> Self {
        self.params.provider_data = Some(provider_data.into());
        self
    }

    ///URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
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

    ///Pass *True* if the user's phone number should be sent to provider
    pub fn send_phone_number_to_provider(
        mut self,
        send_phone_number_to_provider: impl Into<bool>,
    ) -> Self {
        self.params.send_phone_number_to_provider = send_phone_number_to_provider.into();
        self
    }

    ///Pass *True* if the user's email address should be sent to provider
    pub fn send_email_to_provider(mut self, send_email_to_provider: impl Into<bool>) -> Self {
        self.params.send_email_to_provider = send_email_to_provider.into();
        self
    }

    ///Pass *True* if the final price depends on the shipping method
    pub fn is_flexible(mut self, is_flexible: impl Into<bool>) -> Self {
        self.params.is_flexible = is_flexible.into();
        self
    }

    ///Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub fn disable_notification(mut self, disable_notification: impl Into<bool>) -> Self {
        self.params.disable_notification = disable_notification.into();
        self
    }

    ///Protects the contents of the sent message from forwarding and saving
    pub fn protect_content(mut self, protect_content: impl Into<bool>) -> Self {
        self.params.protect_content = protect_content.into();
        self
    }

    ///If the message is a reply, ID of the original message
    pub fn reply_to_message_id(mut self, reply_to_message_id: impl Into<i64>) -> Self {
        self.params.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }

    ///Pass *True* if the message should be sent even if the specified replied-to message is not found
    pub fn allow_sending_without_reply(
        mut self,
        allow_sending_without_reply: impl Into<bool>,
    ) -> Self {
        self.params.allow_sending_without_reply = allow_sending_without_reply.into();
        self
    }

    ///A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If empty, one 'Pay `total price`' button will be shown. If not empty, the first button must be a Pay button.
    pub fn reply_markup(mut self, reply_markup: impl Into<InlineKeyboardMarkup>) -> Self {
        self.params.reply_markup = Some(reply_markup.into());
        self
    }
}

impl<'a> API {
    ///Use this method to send invoices. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    pub fn send_invoice(
        &'a self,
        chat_id: impl Into<ChatId>,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        provider_token: impl Into<String>,
        currency: impl Into<String>,
        prices: impl Into<Vec<LabeledPrice>>,
    ) -> SendInvoiceRequest {
        SendInvoiceRequest::new(
            self,
            chat_id.into(),
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
