use std::{fmt::Debug, future::IntoFuture, sync::atomic::AtomicI64, time::Duration};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    client::ApiClient,
    entities::{
        misc::{chat_id::ChatId, input_file::GetFiles},
        update::{AllowedUpdates, Update},
    },
    errors::{ConogramError, ConogramErrorType, TgApiError},
    methods::{
        edit_message_caption::EditMessageCaptionRequest, edit_message_text::EditMessageTextRequest,
        send_animation::SendAnimationRequest, send_audio::SendAudioRequest,
        send_contact::SendContactRequest, send_dice::SendDiceRequest,
        send_document::SendDocumentRequest, send_game::SendGameRequest,
        send_invoice::SendInvoiceRequest, send_location::SendLocationRequest,
        send_media_group::SendMediaGroupRequest, send_message::SendMessageRequest,
        send_photo::SendPhotoRequest, send_poll::SendPollRequest, send_sticker::SendStickerRequest,
        send_venue::SendVenueRequest, send_video::SendVideoRequest,
        send_video_note::SendVideoNoteRequest, send_voice::SendVoiceRequest,
    },
    request::RequestT,
    server_config::ApiServerConfig,
};

macro_rules! set_default_param {
    ($api_client: expr, $param_name: literal, $value: ident, [$($request: ty),*] $(,)?) => {
        $(
            $api_client.set_default_request_param(
                <$request>::get_name(),
                $param_name,
                $value.clone()
            )?;
        )*
    }
}

// /// Wraps API request into [``Api::request_ref(self)``]
// pub trait WrapRequest {
//     fn wrap(&self) -> impl Future<Output = Result<Self::ReturnType, ConogramError>>
//     where
//         Self: RequestT,
//         for<'a> &'a Self: IntoFuture<Output = Result<Self::ReturnType, ConogramError>>,
//         for<'a> <&'a Self as IntoFuture>::IntoFuture: Send,
//         Self::ReturnType: std::marker::Send,
//     {
//         Api::request_ref(self)
//     }
// }
// impl<T> WrapRequest for T {}

pub struct ApiToken(String);
impl ApiToken {
    pub(crate) fn leak(&self) -> &str {
        &self.0
    }
}
impl Debug for ApiToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut token_splits = self.0.split(':');
        f.debug_struct("ApiToken")
            .field("bot_id", &token_splits.next().unwrap_or("Unknown"))
            .field(
                "token",
                &format!("{}...", &token_splits.next().unwrap_or("Unknown")[..6]),
            )
            .finish()
    }
}

impl<T: AsRef<str>> From<T> for ApiToken {
    fn from(value: T) -> Self {
        Self(value.as_ref().to_string())
    }
}

pub struct ApiConfig {
    pub token: ApiToken,
    pub server_config: ApiServerConfig,
}

impl ApiConfig {
    pub fn new(bot_token: impl Into<ApiToken>, server_config: Option<ApiServerConfig>) -> Self {
        Self {
            token: bot_token.into(),
            server_config: server_config.unwrap_or_else(|| ApiServerConfig::remote(false)),
        }
    }

    pub fn remote(bot_token: impl Into<ApiToken>, use_test_env: bool) -> Self {
        Self::new(bot_token, Some(ApiServerConfig::remote(use_test_env)))
    }

    pub fn local(
        bot_token: impl Into<ApiToken>,
        server_url: impl Into<String>,
        use_test_env: bool,
    ) -> Self {
        Self::new(
            bot_token,
            Some(ApiServerConfig::new(server_url.into(), use_test_env)),
        )
    }
}

pub struct Api {
    api_client: ApiClient,

    allowed_updates: Vec<String>,
    get_updates_offset: AtomicI64,
    polling_timeout: u64,
}

impl Api {
    #[must_use]
    pub fn new(config: ApiConfig) -> Self {
        Self {
            api_client: ApiClient::new(config),

            allowed_updates: vec![],
            get_updates_offset: AtomicI64::new(0),
            polling_timeout: 600,
        }
    }

    /// Sets `allow_sending_without_reply` to `true` for all applicable requests
    pub fn set_essential_request_defaults(&mut self) -> Result<(), ConogramErrorType> {
        set_default_param!(
            self.api_client,
            "allow_sending_without_reply",
            true,
            [
                // crate::entities::reply_parameters::ReplyParameters,
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

    /// Sets the timeout for getUpdates request
    pub fn set_polling_timeout(&mut self, timeout_secs: u64) {
        self.polling_timeout = timeout_secs;
    }

    /// Set default value for particular request
    pub fn set_default_request_param(
        &mut self,
        method: impl Into<String>,
        param_name: impl Into<String>,
        value: impl Serialize,
    ) -> Result<(), ConogramErrorType> {
        self.api_client
            .set_default_request_param(method.into(), param_name, value)
    }

    /// Sets default `parse_mode` for applicable requests
    pub fn set_parse_mode(&mut self, value: impl Into<String>) -> Result<(), ConogramErrorType> {
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

    /// Sets default `link_preview_options` for applicable requests
    pub fn set_default_link_preview(
        &mut self,
        value: impl Into<crate::entities::link_preview_options::LinkPreviewOptions>,
    ) -> Result<(), ConogramErrorType> {
        let value = value.into();

        set_default_param!(
            self.api_client,
            "link_preview_options",
            value,
            [SendMessageRequest, EditMessageTextRequest]
        );

        Ok(())
    }

    /// Set allowed update kinds list which will be later used in polling
    pub fn set_allowed_updates(
        &mut self,
        allowed_updates: impl IntoIterator<Item = AllowedUpdates>,
    ) {
        self.allowed_updates = allowed_updates.into_iter().map(|x| x.to_string()).collect();
    }

    /// Internal conogram method. Returns ``Ok(false)`` instead of `Err` if the message can't be deleted
    pub async fn delete_message_exp(
        &self,
        chat_id: impl Into<ChatId> + Send,
        message_id: impl Into<i64> + Send,
    ) -> Result<bool, ConogramError> {
        let result = self.delete_message(chat_id, message_id).wrap().await;
        if let Err(err) = &result {
            if let ConogramErrorType::ApiError(_) = &err.error {
                return Ok(false);
            }
        }
        result
    }

    /// Internal conogram method. Returns ``Ok(false)`` instead of `Err` if one or more of the messages can't be deleted
    pub async fn delete_messages_exp(
        &self,
        chat_id: impl Into<ChatId> + Send,
        message_ids: impl IntoIterator<Item = impl Into<i64>> + Send,
    ) -> Result<bool, ConogramError> {
        let result = self.delete_messages(chat_id, message_ids).wrap().await;
        if let Err(err) = &result {
            if let ConogramErrorType::ApiError(_) = &err.error {
                return Ok(false);
            }
        }
        result
    }

    /// Poll the server for pending updates
    pub async fn poll_once(&self) -> Result<Vec<Update>, ConogramError> {
        let offset = self
            .get_updates_offset
            .load(std::sync::atomic::Ordering::Relaxed);

        let r = self
            .get_updates()
            .allowed_updates(self.allowed_updates.clone())
            .offset(offset)
            .timeout(self.polling_timeout.clamp(0, i64::MAX as u64) as i64);

        let updates = r.await?;
        if let Some(last_update) = updates.iter().max_by_key(|update| update.update_id) {
            self.get_updates_offset.store(
                last_update.update_id + 1,
                std::sync::atomic::Ordering::Relaxed,
            );
        }
        Ok(updates)
    }

    pub async fn method_json<
        ReturnType: DeserializeOwned + std::fmt::Debug,
        Params: Serialize + Sync + std::fmt::Debug,
    >(
        &self,
        method: &str,
        params: Option<&Params>,
    ) -> Result<ReturnType, ConogramError> {
        self.api_client.method_json(method, params).await
    }

    pub async fn method_multipart_form<
        ReturnType: DeserializeOwned + std::fmt::Debug,
        Params: Serialize + GetFiles + Sync + std::fmt::Debug,
    >(
        &self,
        method: &str,
        params: Option<&Params>,
    ) -> Result<ReturnType, ConogramError> {
        self.api_client.method_multipart_form(method, params).await
    }

    /// Same as [`Api::request_ref`] but takes `request` by value
    pub async fn request<Request: RequestT + Send + Sync>(
        request: Request,
    ) -> Result<Request::ReturnType, ConogramError>
    where
        for<'a> &'a Request: IntoFuture<Output = Result<Request::ReturnType, ConogramError>> + Send,
        for<'a> <&'a Request as IntoFuture>::IntoFuture: Send,
        Request::ReturnType: Send,
    {
        Self::request_ref::<Request>(&request).await
    }

    /// This will make API request and automativally handle some common errors like `RetryAfter`, `BadGateway` and `GatewayTimeout`
    pub async fn request_ref<Request: RequestT + Sync>(
        request: &Request,
    ) -> Result<Request::ReturnType, ConogramError>
    where
        for<'a> &'a Request: IntoFuture<Output = Result<Request::ReturnType, ConogramError>>,
        for<'a> <&'a Request as IntoFuture>::IntoFuture: Send,
    {
        let mut result = request.await;

        let mut wait_for = 1;

        while !match &result {
            Err(err) => {
                if let ConogramErrorType::ApiError(error) = &err.error {
                    match error {
                        TgApiError::RetryAfter(params) => {
                            if let Some(params) = params.parameters.as_ref() {
                                tokio::time::sleep(Duration::from_secs(
                                    params.retry_after.unwrap_or_default() as u64,
                                ))
                                .await;
                                result = request.await;
                                false
                            } else {
                                true
                            }
                        }
                        TgApiError::BadGateway(_) | TgApiError::GatewayTimeout(_) => {
                            wait_for = std::cmp::min(wait_for * 2, 60);
                            tokio::time::sleep(Duration::from_secs(wait_for)).await;
                            result = request.await;
                            false
                        }
                        _ => true,
                    }
                } else {
                    true
                }
            }

            _ => true,
        } {}

        result
    }
}
