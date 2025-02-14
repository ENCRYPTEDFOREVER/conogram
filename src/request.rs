use std::{
    any::Any,
    fmt::Debug,
    future::{Future, IntoFuture},
    time::Duration,
};

// use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    api::Api,
    entities::misc::{chat_id::ChatId, input_file::GetFiles},
    errors::{ConogramError, ConogramErrorType, TgApiError},
};

pub trait TargetChatId {
    fn get_target_chat_id(&self) -> Option<ChatId>;
}

pub trait RequestT
where
    Self: Sized + Send + Sync,
{
    type ParamsType: Serialize + TargetChatId + Debug + Send + Sync + Any;
    type ReturnType: DeserializeOwned + Debug + Send + Sync + Clone + Any;

    /// Request name as defined by the Telegram Bot API, e.g. ``sendMessage``
    fn get_name() -> &'static str;

    fn get_api_ref(&self) -> &Api;
    fn get_params_ref(&self) -> &Self::ParamsType;

    /// Same as [`send_ref()`](RequestT::send_ref) but consumes `Self`
    fn send(self) -> impl Future<Output = Result<Self::ReturnType, ConogramError>> + Send {
        async move { self.send_ref().await }
    }

    /// Execute the request
    fn send_ref(&self) -> impl Future<Output = Result<Self::ReturnType, ConogramError>> + Send {
        async {
            self.get_api_ref()
                .method_json(Self::get_name(), Some(self.get_params_ref()))
                .await
        }
    }

    /// Same as [`send_multipart_ref()`](RequestT::send_multipart_ref) but consumes `Self`
    fn send_multipart(self) -> impl Future<Output = Result<Self::ReturnType, ConogramError>>
    where
        Self::ParamsType: GetFiles,
    {
        async move { self.send_multipart_ref().await }
    }

    /// Execute the request, which needs multipart upload
    fn send_multipart_ref(&self) -> impl Future<Output = Result<Self::ReturnType, ConogramError>>
    where
        Self::ParamsType: GetFiles,
    {
        async {
            self.get_api_ref()
                .method_multipart_form(Self::get_name(), Some(self.get_params_ref()))
                .await
        }
    }

    /// Execute the request, automatically handling flood wait and BadGateway, GatewayTimeout errors
    fn wrap(&self) -> impl Future<Output = Result<Self::ReturnType, ConogramError>> + Send
    where
        for<'a> &'a Self: IntoFuture<Output = Result<Self::ReturnType, ConogramError>>,
        for<'a> <&'a Self as IntoFuture>::IntoFuture: Send,
    {
        async move {
            let mut result = self.await;

            let mut wait_for = 1;

            while !match &result {
                Err(err) => {
                    if let ConogramErrorType::ApiError(error) = &err.error {
                        match error {
                            TgApiError::RetryAfter(params) => {
                                if let Some(params) = params.parameters.as_ref() {
                                    let retry_after = params.retry_after.unwrap_or_default();
                                    if retry_after > 0 {
                                        let mut retry_after = retry_after as u64;
                                        log::debug!(
                                            "Got RetryAfter {retry_after}s in {}",
                                            Self::get_name()
                                        );

                                        if retry_after > 600 {
                                            log::warn!("Unusually high RetryAfter: {retry_after}s, clamping to 600s");
                                            retry_after = 600;
                                        }

                                        self.get_api_ref()
                                            .register_flood_wait_hit(self, retry_after);
                                        tokio::time::sleep(Duration::from_secs(retry_after)).await;
                                    } else {
                                        log::warn!("RetryAfter is negative: {retry_after}");
                                    }

                                    result = self.await;
                                    false
                                } else {
                                    true
                                }
                            }
                            TgApiError::BadGateway(_) | TgApiError::GatewayTimeout(_) => {
                                wait_for = std::cmp::min(wait_for * 2, 60);
                                log::debug!("Got gateway error, retrying in {wait_for}s");
                                tokio::time::sleep(Duration::from_secs(wait_for)).await;
                                result = self.await;
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

    /// The same as [RequestT::wrap()] except the request will not be called if there is ongoing flood wait for this request
    ///
    /// Unlike [`RequestT::wrap_nr_o()`][RequestT::wrap_nr_o()] wraps only request's ReturnType in an Option, i.e. Returns a **Result<`Option<` RequestReturnType`>`>**
    ///
    /// See: [``Api::get_flood_wait_duration()``][Api::get_flood_wait_duration]
    fn wrap_nr(
        &self,
    ) -> impl Future<Output = Result<Option<Self::ReturnType>, ConogramError>> + Send
    where
        for<'a> &'a Self: IntoFuture<Output = Result<Self::ReturnType, ConogramError>>,
        for<'a> <&'a Self as IntoFuture>::IntoFuture: Send,
    {
        async move {
            if let Some(wait_for) = self.get_api_ref().get_flood_wait_duration(self) {
                log::debug!(
                    "Skipped {} in chat {:?}, RetryAfter: {wait_for:?}",
                    Self::get_name(),
                    self.get_params_ref().get_target_chat_id()
                );
                return Ok(None);
            }

            self.wrap().await.map(Some)
        }
    }

    /// The same as [RequestT::wrap()] except the request will not be called if there is ongoing flood wait for this request
    ///
    /// Wraps the entire request in an Option, i.e. Returns an **`Option<`Result&lt;RequestReturnType&gt;`>`**
    ///
    /// See: [``Api::get_flood_wait_duration()``][Api::get_flood_wait_duration]
    fn wrap_nr_o(
        &self,
    ) -> impl Future<Output = Option<Result<Self::ReturnType, ConogramError>>> + Send
    where
        for<'a> &'a Self: IntoFuture<Output = Result<Self::ReturnType, ConogramError>>,
        for<'a> <&'a Self as IntoFuture>::IntoFuture: Send,
    {
        async move {
            if let Some(wait_for) = self.get_api_ref().get_flood_wait_duration(self) {
                log::debug!(
                    "Skipped {} in chat {:?}, RetryAfter: {wait_for:?}",
                    Self::get_name(),
                    self.get_params_ref().get_target_chat_id()
                );
                return None;
            }

            Some(self.wrap().await)
        }
    }

    /// The same as [RequestT::wrap()] except the request will not be called if there is ongoing flood wait for this request and it's longer than `threshold`
    ///
    /// Unlike [`RequestT::wrap_nr_o()`][RequestT::wrap_nr_thr_o()] wraps only request's ReturnType in an Option, i.e. Returns a **Result<`Option<` RequestReturnType`>`>**
    ///
    /// See: [``Api::get_flood_wait_duration()``][Api::get_flood_wait_duration]
    fn wrap_nr_thr(
        &self,
        threshold: impl Into<Duration>,
    ) -> impl Future<Output = Result<Option<Self::ReturnType>, ConogramError>> + Send
    where
        for<'a> &'a Self: IntoFuture<Output = Result<Self::ReturnType, ConogramError>>,
        for<'a> <&'a Self as IntoFuture>::IntoFuture: Send,
    {
        let threshold = threshold.into();
        async move {
            if let Some(wait_for) = self.get_api_ref().get_flood_wait_duration(self) {
                if wait_for > threshold {
                    log::debug!(
                        "Skipped {} in chat {:?}, RetryAfter: {wait_for:?} > {threshold:?}",
                        Self::get_name(),
                        self.get_params_ref().get_target_chat_id()
                    );
                    return Ok(None);
                }
            }
            self.wrap().await.map(Some)
        }
    }

    /// The same as [RequestT::wrap()] except the request will not be called if there is ongoing flood wait for this request and it's longer than `threshold`
    ///
    /// Wraps the entire request in an Option, i.e. Returns an **`Option<`Result&lt;RequestReturnType&gt;`>`**
    ///
    /// See: [``Api::get_flood_wait_duration()``][Api::get_flood_wait_duration]
    fn wrap_nr_thr_o(
        &self,
        threshold: impl Into<Duration>,
    ) -> impl Future<Output = Option<Result<Self::ReturnType, ConogramError>>> + Send
    where
        for<'a> &'a Self: IntoFuture<Output = Result<Self::ReturnType, ConogramError>>,
        for<'a> <&'a Self as IntoFuture>::IntoFuture: Send,
    {
        let threshold = threshold.into();
        async move {
            if let Some(wait_for) = self.get_api_ref().get_flood_wait_duration(self) {
                if wait_for > threshold {
                    log::debug!(
                        "Skipped {} in chat {:?}, RetryAfter: {wait_for:?} > {threshold:?}",
                        Self::get_name(),
                        self.get_params_ref().get_target_chat_id()
                    );
                    return None;
                }
            }
            Some(self.wrap().await)
        }
    }
}
