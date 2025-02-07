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
    entities::misc::input_file::GetFiles,
    errors::{ConogramError, ConogramErrorType, TgApiError},
};

// #[async_trait]
pub trait RequestT
where
    Self: Sized + Send + Sync,
{
    type ParamsType: Serialize + Debug + Send + Sync + Any;
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

    /// Execute the request, automatically handling rate limits, BadGateway, GatewayTimeout (when Bot API server restarts)
    fn wrap(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::ReturnType, ConogramError>> + Send
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
                                    tokio::time::sleep(Duration::from_secs(
                                        params.retry_after.unwrap_or_default() as u64,
                                    ))
                                    .await;
                                    result = self.await;
                                    false
                                } else {
                                    true
                                }
                            }
                            TgApiError::BadGateway(_) | TgApiError::GatewayTimeout(_) => {
                                wait_for = std::cmp::min(wait_for * 2, 60);
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
}
