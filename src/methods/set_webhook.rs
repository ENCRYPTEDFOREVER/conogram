use crate::api::API;
use crate::entities::misc::input_file::GetFiles;
use crate::entities::misc::input_file::InputFile;
use crate::entities::misc::input_file::Moose;
use crate::errors::ConogramError;
use crate::impl_into_future_multipart;
use crate::request::RequestT;
use crate::utils::deserialize_utils::is_false;
use serde::Serialize;
use std::collections::HashMap;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetWebhookParams {
    pub url: String,
    #[serde(skip, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allowed_updates: Vec<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub drop_pending_updates: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

impl GetFiles for SetWebhookParams {
    fn get_files(&self) -> HashMap<Moose, &InputFile> {
        let mut map = HashMap::new();
        if let Some(certificate) = &self.certificate {
            map.insert(Moose::Owned("certificate".into()), certificate);
        }
        map
    }
}
impl_into_future_multipart!(SetWebhookRequest<'a>);

///Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized [Update](https://core.telegram.org/bots/api/#update). In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns *True* on success.
///
///If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter *secret\_token*. If specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret token as content.
#[derive(Clone)]
pub struct SetWebhookRequest<'a> {
    api: &'a API,
    params: SetWebhookParams,
}

impl<'a> RequestT for SetWebhookRequest<'a> {
    type ParamsType = SetWebhookParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setWebhook"
    }
    fn get_api_ref(&self) -> &API {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        true
    }
}
impl<'a> SetWebhookRequest<'a> {
    pub fn new(api: &'a API, url: impl Into<String>) -> Self {
        Self {
            api,
            params: SetWebhookParams {
                url: url.into(),
                certificate: Option::default(),
                ip_address: Option::default(),
                max_connections: Option::default(),
                allowed_updates: Vec::default(),
                drop_pending_updates: bool::default(),
                secret_token: Option::default(),
            },
        }
    }

    ///HTTPS URL to send updates to. Use an empty string to remove webhook integration
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.params.url = url.into();
        self
    }

    ///Upload your public key certificate so that the root certificate in use can be checked. See our [self-signed guide](https://core.telegram.org/bots/self-signed) for details.
    pub fn certificate(mut self, certificate: impl Into<InputFile>) -> Self {
        self.params.certificate = Some(certificate.into());
        self
    }

    ///The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    pub fn ip_address(mut self, ip_address: impl Into<String>) -> Self {
        self.params.ip_address = Some(ip_address.into());
        self
    }

    ///The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to *40*. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    pub fn max_connections(mut self, max_connections: impl Into<i64>) -> Self {
        self.params.max_connections = Some(max_connections.into());
        self
    }

    ///A JSON-serialized list of the update types you want your bot to receive. For example, specify `["message", "edited_channel_post", "callback_query"]` to only receive updates of these types. See [Update](https://core.telegram.org/bots/api/#update) for a complete list of available update types. Specify an empty list to receive all update types except *chat\_member*, *message\_reaction*, and *message\_reaction\_count* (default). If not specified, the previous setting will be used.  
    ///Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.
    pub fn allowed_updates(
        mut self,
        allowed_updates: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.params.allowed_updates = allowed_updates.into_iter().map(Into::into).collect();
        self
    }

    ///Pass *True* to drop all pending updates
    pub fn drop_pending_updates(mut self, drop_pending_updates: impl Into<bool>) -> Self {
        self.params.drop_pending_updates = drop_pending_updates.into();
        self
    }

    ///A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token” in every webhook request, 1-256 characters. Only characters `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed. The header is useful to ensure that the request comes from a webhook set by you.
    pub fn secret_token(mut self, secret_token: impl Into<String>) -> Self {
        self.params.secret_token = Some(secret_token.into());
        self
    }
}

impl<'a> API {
    ///Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized [Update](https://core.telegram.org/bots/api/#update). In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns *True* on success.
    ///
    ///If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter *secret\_token*. If specified, the request will contain a header “X-Telegram-Bot-Api-Secret-Token” with the secret token as content.
    pub fn set_webhook(&'a self, url: impl Into<String>) -> SetWebhookRequest {
        SetWebhookRequest::new(self, url)
    }
}

// Divider: all content below this line will be preserved after code regen
