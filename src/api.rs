use std::fmt::Debug;
use std::future::IntoFuture;
use std::sync::atomic::AtomicI64;
use std::time::Duration;

use crate::client::ApiClient;
use crate::entities::misc::input_file::GetFiles;
use crate::entities::update::{AllowedUpdates, Update};
use crate::errors::{Error, TgApiError};
use crate::methods::edit_message_caption::EditMessageCaptionRequest;
use crate::methods::edit_message_text::EditMessageTextRequest;
use crate::methods::send_animation::SendAnimationRequest;
use crate::methods::send_audio::SendAudioRequest;
use crate::methods::send_contact::SendContactRequest;
use crate::methods::send_dice::SendDiceRequest;
use crate::methods::send_document::SendDocumentRequest;
use crate::methods::send_game::SendGameRequest;
use crate::methods::send_invoice::SendInvoiceRequest;
use crate::methods::send_location::SendLocationRequest;
use crate::methods::send_media_group::SendMediaGroupRequest;
use crate::methods::send_message::SendMessageRequest;
use crate::methods::send_photo::SendPhotoRequest;
use crate::methods::send_poll::SendPollRequest;
use crate::methods::send_sticker::SendStickerRequest;
use crate::methods::send_venue::SendVenueRequest;
use crate::methods::send_video::SendVideoRequest;
use crate::methods::send_video_note::SendVideoNoteRequest;
use crate::methods::send_voice::SendVoiceRequest;
use crate::request::RequestT;
use crate::server_config::ApiServerConfig;

use serde::de::DeserializeOwned;
use serde::Serialize;

macro_rules! set_default_param {
    ($api_client: expr, $param_name: literal, $value: ident, [$($request: ty),*]) => {
        $(
            $api_client.set_default_request_param(
                <$request>::get_name(),
                $param_name,
                $value.clone()
            )?;
        )*
    }
}

#[derive(Clone)]
pub struct APIConfig {
    pub token: String,
    pub server_config: ApiServerConfig,
}

impl APIConfig {
    pub fn new(bot_token: impl ToString, server_config: Option<ApiServerConfig>) -> Self {
        Self {
            token: bot_token.to_string(),
            server_config: server_config.unwrap_or(ApiServerConfig::remote(false)),
        }
    }

    pub fn remote(bot_token: impl ToString, use_test_env: bool) -> Self {
        Self::new(bot_token, Some(ApiServerConfig::remote(use_test_env)))
    }

    pub fn local(
        bot_token: impl ToString,
        server_url: impl Into<String>,
        port: impl Into<u16>,
        use_test_env: bool,
    ) -> Self {
        Self::new(
            bot_token,
            Some(ApiServerConfig::new(
                server_url.into(),
                port.into(),
                use_test_env,
            )),
        )
    }
}

pub struct API {
    config: APIConfig,
    api_client: ApiClient,

    allowed_updates: Vec<String>,
    get_updates_offset: AtomicI64,
}

impl Debug for API {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut token_splits = self.config.token.split(':');
        f.debug_struct("API")
            .field("bot_id", &token_splits.next().unwrap_or("Unknown"))
            .field(
                "token",
                &format!("{}...", &token_splits.next().unwrap_or("Unknown")[..10]),
            )
            .finish()
    }
}

impl API {
    pub fn new(config: APIConfig) -> Self {
        Self {
            config: config.clone(),
            api_client: ApiClient::new(config),

            allowed_updates: vec![],
            get_updates_offset: AtomicI64::new(0),
        }
    }

    /// Sets `allow_sending_without_reply` to `true` for all requests
    pub fn set_essential_request_defaults(&mut self) -> Result<(), Error> {
        set_default_param!(
            self.api_client,
            "allow_sending_without_reply",
            true,
            [
                SendMessageRequest,
                SendPhotoRequest,
                SendAnimationRequest,
                SendAudioRequest,
                SendDocumentRequest,
                SendVoiceRequest,
                SendContactRequest,
                SendDiceRequest,
                SendGameRequest,
                SendInvoiceRequest,
                SendStickerRequest,
                SendVenueRequest,
                SendVideoRequest,
                SendLocationRequest,
                SendVideoNoteRequest,
                SendMediaGroupRequest
            ]
        );

        Ok(())
    }

    /// Set default value for particular request
    pub fn set_default_request_param(
        &mut self,
        method: impl Into<String>,
        param_name: impl Into<String>,
        value: impl Serialize,
    ) -> Result<(), Error> {
        self.api_client
            .set_default_request_param(method.into(), param_name, value)
    }

    /// Sets default `parse_mode` for applicable requests
    pub fn set_parse_mode(&mut self, value: impl Into<String>) -> Result<(), Error> {
        let value = value.into();

        set_default_param!(
            self.api_client,
            "parse_mode",
            value,
            [
                SendMessageRequest,
                SendPhotoRequest,
                SendAnimationRequest,
                SendAudioRequest,
                SendDocumentRequest,
                SendVoiceRequest,
                EditMessageTextRequest,
                EditMessageCaptionRequest
            ]
        );

        self.api_client.set_default_request_param(
            SendPollRequest::get_name(),
            "explanation_parse_mode",
            value,
        )?;

        Ok(())
    }

    pub async fn method_json<
        ReturnType: DeserializeOwned + std::fmt::Debug,
        Params: Serialize + std::fmt::Debug,
    >(
        &self,
        method: &str,
        params: Option<&Params>,
    ) -> Result<ReturnType, Error> {
        self.api_client.method_json(method, params).await
    }

    pub async fn method_multipart_form<
        ReturnType: DeserializeOwned + std::fmt::Debug,
        Params: Serialize + GetFiles + std::fmt::Debug,
    >(
        &self,
        method: &str,
        params: Option<&Params>,
    ) -> Result<ReturnType, Error> {
        self.api_client.method_multipart_form(method, params).await
    }

    /// Set allowed update kinds list which will be later used in polling
    pub fn set_allowed_updates(&mut self, allowed_updates: Vec<AllowedUpdates>) {
        self.allowed_updates = allowed_updates.iter().map(|x| x.to_string()).collect();
    }

    /// Poll the server for pending updates
    pub async fn poll_once(&self) -> Result<Vec<Update>, Error> {
        let offset = self
            .get_updates_offset
            .load(std::sync::atomic::Ordering::Relaxed);
        let r = self
            .get_updates()
            .allowed_updates(self.allowed_updates.clone())
            .offset(offset)
            .timeout(60);

        let updates = r.await?;
        if let Some(last_update) = updates.iter().max_by_key(|update| update.update_id) {
            self.get_updates_offset.store(
                last_update.update_id + 1,
                std::sync::atomic::Ordering::Relaxed,
            );
        }
        Ok(updates)
    }

    /// Same as [`API::request_ref`] but will consume the request
    pub async fn request<Request, ReturnType>(request: Request) -> Result<ReturnType, Error>
    where
        for<'a> &'a Request: IntoFuture<Output = Result<ReturnType, Error>>,
    {
        Self::request_ref(&request).await
    }

    /// This will make API request and automativally handle some common errors like RetryAfter or BadGateway
    pub async fn request_ref<Request, ReturnType>(request: &Request) -> Result<ReturnType, Error>
    where
        for<'a> &'a Request: IntoFuture<Output = Result<ReturnType, Error>>,
    {
        let mut result = request.await;

        let mut wait_for = 1;

        while !match &result {
            Err(Error::ApiError(TgApiError::RetryAfter(wait_for))) => {
                tokio::time::sleep(Duration::from_secs(*wait_for as u64)).await;
                result = request.await;
                false
            }
            Err(Error::ApiError(TgApiError::BadGateway))
            | Err(Error::ApiError(TgApiError::GatewayTimeout)) => {
                wait_for = std::cmp::min(wait_for * 2, 60);
                tokio::time::sleep(Duration::from_secs(wait_for)).await;
                result = request.await;
                false
            }
            _ => true,
        } {}

        result
    }
}
