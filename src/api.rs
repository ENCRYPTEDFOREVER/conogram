use std::{
    any::{Any, TypeId},
    fmt::Debug,
    future::IntoFuture,
    sync::atomic::AtomicI64,
    time::{Duration, Instant},
};

use dashmap::DashMap;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    chat_member_cache::ChatMemberCache,
    client::ApiClient,
    entities::{
        chat_member::ChatMember,
        misc::{chat_id::ChatId, input_file::GetFiles},
        update::{AllowedUpdates, Update},
    },
    errors::{ConogramError, ConogramErrorType, TgApiError},
    methods::{
        edit_message_caption::EditMessageCaptionRequest, edit_message_text::EditMessageTextRequest,
        get_chat_member::GetChatMemberParams, send_animation::SendAnimationRequest,
        send_audio::SendAudioRequest, send_contact::SendContactRequest, send_dice::SendDiceRequest,
        send_document::SendDocumentRequest, send_game::SendGameRequest,
        send_invoice::SendInvoiceRequest, send_location::SendLocationRequest,
        send_media_group::SendMediaGroupRequest, send_message::SendMessageRequest,
        send_photo::SendPhotoRequest, send_poll::SendPollRequest, send_sticker::SendStickerRequest,
        send_venue::SendVenueRequest, send_video::SendVideoRequest,
        send_video_note::SendVideoNoteRequest, send_voice::SendVoiceRequest,
    },
    request::{RequestT, TargetChatId},
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

pub struct ApiToken(String);
impl ApiToken {
    pub(crate) fn leak(&self) -> &str {
        self.0.as_str()
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

impl Debug for ApiConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiConfig")
            .field(
                "bot_id",
                &self.token.leak().split(':').next().unwrap_or("Unknown"),
            )
            .field("server_config", &self.server_config)
            .finish()
    }
}

pub struct Api {
    api_client: ApiClient,

    request_stats_enabled: bool,
    request_stats: DashMap<String, usize>,

    chat_member_cache: Option<ChatMemberCache>,

    flood_wait_hits: DashMap<(String, Option<ChatId>), (Instant, Duration)>,

    allowed_updates: Vec<String>,
    get_updates_offset: AtomicI64,
    polling_timeout: u64,
}

impl Debug for Api {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Api")
            .field("api_client", &self.api_client)
            .field("request_stats_enabled", &self.request_stats_enabled)
            .field("allowed_updates", &self.allowed_updates)
            .field("get_updates_offset", &self.get_updates_offset)
            .field("polling_timeout", &self.polling_timeout)
            .finish_non_exhaustive()
    }
}

impl Api {
    #[must_use]
    pub fn new(config: ApiConfig) -> Self {
        Self {
            api_client: ApiClient::new(config),

            allowed_updates: vec![],
            get_updates_offset: AtomicI64::new(0),
            polling_timeout: 600,

            chat_member_cache: None,
            request_stats: DashMap::new(),
            request_stats_enabled: false,

            flood_wait_hits: DashMap::new(),
        }
    }

    /// Enable [ChatMember] caching from updates and requests
    ///
    /// Notes:
    /// * Disabled by default
    /// * Stored inside this [Api] instance
    /// * [ChatMember](AllowedUpdates::ChatMember) update will be automatically enabled if you enable caching
    pub fn set_chat_member_cache_enabled(&mut self, enabled: bool) {
        self.chat_member_cache = if enabled {
            Some(ChatMemberCache::default())
        } else {
            None
        };

        self.check_allowed_updates();
    }

    /// Enable request count statistics collection, which can be retrieved using [Api::get_request_stats()]
    ///
    /// Notes:
    /// * Disabled by default
    /// * Stored inside this [Api] instance
    /// * Disabling after it was enabled will not reset previous statistics
    pub const fn set_request_stats_enabled(&mut self, enabled: bool) {
        self.request_stats_enabled = enabled;
    }

    /// Returns API request count statistics as DashMap<RequestName, CallCount>
    ///
    /// Note:
    /// * [Api::set_request_stats_enabled()] must be called to enable stats collection
    pub fn get_request_stats(&self) -> DashMap<String, usize> {
        if !self.request_stats_enabled {
            log::warn!("Called get_request_stats() with disabled statistics collection");
        }
        self.request_stats.clone()
    }

    pub(crate) fn register_flood_wait_hit<Request: RequestT>(
        &self,
        request: &Request,
        retry_after: u64,
    ) {
        let request_name = Request::get_name();
        let target_chat_id = if request_name.contains("message") {
            request.get_params_ref().get_target_chat_id()
        } else {
            None
        };

        self.flood_wait_hits.insert(
            (request_name.into(), target_chat_id),
            (Instant::now(), Duration::from_secs(retry_after)),
        );
    }

    /// Return a [Duration], representing remaining flood wait time for some certain request with some certain parameters
    ///
    /// Notes:
    /// * As stated in the description, hitting flood wait is dependent both on request and it's parameters
    /// * For this to work correctly you must use [RequestT::wrap] for API requests, as it handles tracking rate limits
    pub fn get_flood_wait_duration<Request: RequestT>(
        &self,
        request: &Request,
    ) -> Option<Duration> {
        let request_name = Request::get_name();
        let target_chat_id = if request_name.contains("message") {
            request.get_params_ref().get_target_chat_id()
        } else {
            None
        };

        if let Some(v) = self
            .flood_wait_hits
            .get(&(request_name.into(), target_chat_id))
        {
            let (hit_instant, wait_for) = *v;
            wait_for.checked_sub(hit_instant.elapsed())
        } else {
            None
        }
    }

    fn check_allowed_updates(&mut self) {
        if self.chat_member_cache.is_some() {
            let chat_member = AllowedUpdates::ChatMember.to_string();
            if !self.allowed_updates.contains(&chat_member) {
                self.allowed_updates.push(chat_member);
            }
        }
    }

    /// Sets `allow_sending_without_reply` to `true` for all applicable requests
    pub fn set_essential_request_defaults(&mut self) -> Result<(), serde_json::Error> {
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

    /// Sets the timeout for [GetUpdatesRequest] request
    pub const fn set_polling_timeout(&mut self, timeout_secs: u64) {
        self.polling_timeout = timeout_secs;
    }

    /// Set default value for particular request
    pub fn set_default_request_param(
        &mut self,
        method: impl Into<String>,
        param_name: impl Into<String>,
        value: impl Serialize,
    ) -> Result<(), serde_json::Error> {
        self.api_client
            .set_default_request_param(method.into(), param_name, value)
    }

    /// Sets default ``parse_mode`` for applicable requests
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
    ) -> Result<(), serde_json::Error> {
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
    ///
    /// Note: [AllowedUpdates::ChatMember] update will be automatically enabled if you enable ChatMember caching
    pub fn set_allowed_updates(
        &mut self,
        allowed_updates: impl IntoIterator<Item = AllowedUpdates>,
    ) {
        self.allowed_updates = allowed_updates.into_iter().map(|x| x.to_string()).collect();
        self.allowed_updates.dedup();
        self.check_allowed_updates();
    }

    /// Internal conogram method. Returns ``Ok(false)`` instead of `Err` if the message can't be deleted
    pub async fn delete_message_exp(
        &self,
        chat_id: impl Into<ChatId> + Send,
        message_id: impl Into<i64> + Send,
    ) -> Result<bool, ConogramError> {
        let result = self.delete_message(chat_id, message_id).wrap().await;
        if let Err(err) = &result {
            if let ConogramErrorType::ApiError(_) = &err.type_ {
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
            if let ConogramErrorType::ApiError(_) = &err.type_ {
                return Ok(false);
            }
        }
        result
    }

    /// Internal method used for caching
    pub fn preprocess_updates(&self, updates: &[Update]) {
        let mut max_update_id = None;
        for update in updates {
            if max_update_id.is_none_or(|prev_max| prev_max < update.update_id) {
                max_update_id = Some(update.update_id);
            }

            if let Some(chat_member_updated) = &update.chat_member {
                if let Some(cache) = &self.chat_member_cache {
                    cache.cache_update(chat_member_updated);
                }
            }
        }

        if let Some(max_update_id) = max_update_id {
            self.get_updates_offset
                .store(max_update_id + 1, std::sync::atomic::Ordering::Relaxed);
        }
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
        self.preprocess_updates(&updates);
        Ok(updates)
    }

    /// Internal method used for API calls
    pub async fn method_json<
        ReturnType: DeserializeOwned + std::fmt::Debug + Clone + Any,
        Params: Serialize + Sync + std::fmt::Debug + Any,
    >(
        &self,
        method: &str,
        params: Option<&Params>,
    ) -> Result<ReturnType, ConogramError> {
        if TypeId::of::<Params>() == TypeId::of::<GetChatMemberParams>()
            && TypeId::of::<ReturnType>() == TypeId::of::<ChatMember>()
        {
            if let Some(cache) = &self.chat_member_cache {
                if let Some(params) = params {
                    let params = unsafe {
                        &*std::ptr::from_ref::<Params>(params).cast::<GetChatMemberParams>()
                    };

                    if let Some(chat_member) = cache.get(&params.chat_id, params.user_id) {
                        let return_type = unsafe {
                            (*(std::ptr::from_ref(&chat_member).cast::<ReturnType>())).clone()
                        };
                        if self.request_stats_enabled {
                            self.request_stats
                                .entry(format!("{method}(CacheHit)"))
                                .and_modify(|n| *n += 1)
                                .or_insert(1);
                        }
                        return Ok(return_type);
                    }

                    if self.request_stats_enabled {
                        self.request_stats
                            .entry(method.to_string())
                            .and_modify(|n| *n += 1)
                            .or_insert(1);
                    }
                    let value: ReturnType =
                        self.api_client.method_json(method, Some(params)).await?;
                    let chat_member = unsafe { &*std::ptr::from_ref(&value).cast::<ChatMember>() };
                    cache.cache_chat_member(&params.chat_id, chat_member);
                    return Ok(value);
                }
            }
        }

        if self.request_stats_enabled {
            self.request_stats
                .entry(method.to_string())
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }
        self.api_client.method_json(method, params).await
    }

    /// Internal method used for API calls which require file uploads
    pub async fn method_multipart_form<
        ReturnType: DeserializeOwned + std::fmt::Debug,
        Params: Serialize + GetFiles + Sync + std::fmt::Debug,
    >(
        &self,
        method: &str,
        params: Option<&Params>,
    ) -> Result<ReturnType, ConogramError> {
        if self.request_stats_enabled {
            self.request_stats
                .entry(method.to_string())
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }

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
                if let ConogramErrorType::ApiError(error) = &err.type_ {
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
