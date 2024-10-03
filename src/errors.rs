#![allow(dead_code)]
use std::fmt::{Debug, Display};

use serde::Deserialize;
use thiserror::Error;

use crate::client::ApiResponse;

#[derive(Error)]
pub struct ConogramError {
    pub method_name: String,
    pub params: serde_json::Value,
    pub error: ConogramErrorType,
}

impl Debug for ConogramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}\nCaused by {}({})",
            self.error, self.method_name, self.params
        ))
    }
}

impl Display for ConogramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}\nCaused by {}({})",
            self.error, self.method_name, self.params
        ))
    }
}

impl ConogramError {
    pub(crate) fn new(
        method_name: impl Into<String>,
        params: impl serde::Serialize + std::fmt::Debug,
        error: ConogramErrorType,
    ) -> Self {
        Self {
            method_name: method_name.into(),
            params: serde_json::to_value(params).unwrap(),
            error,
        }
    }
}

#[derive(Error, Debug)]
pub enum ConogramErrorType {
    /// Errors returned by the Telegram Bot API
    #[error("{0}")]
    ApiError(#[from] TgApiError),

    /// HTTP errors
    #[error("{0}")]
    RequestError(#[from] reqwest::Error),

    /// (De)serialization errors
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),

    /// IO errors
    #[error("{0}")]
    IO(#[from] std::io::Error),
}

#[allow(clippy::fallible_impl_from)]
impl<ReturnType> From<ApiResponse<ReturnType>> for Result<ReturnType, ConogramErrorType> {
    fn from(value: ApiResponse<ReturnType>) -> Self {
        if value.ok {
            Ok(value.result.unwrap())
        } else {
            Err(ConogramErrorType::ApiError(TgApiError::from(value)))
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct TgApiErrorParams {
    pub migrate_to_chat_id: Option<i64>,
    pub retry_after: Option<i64>,
}

/// This object represents errors returned by the Telegram Bot API
#[derive(Error, Debug)]
pub enum TgApiError {
    /// Generic error
    #[error("{0}")]
    Generic(GenericApiErrorParams),

    /// Flood limit reached for this request, retry after N seconds
    #[error("{0}")]
    RetryAfter(GenericApiErrorParams),

    /// Regular http 404, bot token is incorrect or you're trying to access inexistent method
    #[error("{0}")]
    NotFound(GenericApiErrorParams),

    /// Bot token is incorrect
    #[error("{0}")]
    Unauthorized(GenericApiErrorParams),

    /// Another bot instance is running
    #[error("{0}")]
    Conflict(GenericApiErrorParams),

    /// Server-side issues, repeat request later
    #[error("{0}")]
    BadGateway(GenericApiErrorParams),

    /// Server-side issues, repeat request later
    #[error("{0}")]
    GatewayTimeout(GenericApiErrorParams),
}

impl<ReturnType> From<ApiResponse<ReturnType>> for TgApiError {
    fn from(value: ApiResponse<ReturnType>) -> Self {
        match value.error_code {
            Some(error_code) => match error_code {
                401 => Self::Unauthorized(value.into()),
                404 => Self::NotFound(value.into()),
                409 => Self::Conflict(value.into()),
                429 => Self::RetryAfter(value.into()),
                502 => Self::BadGateway(value.into()),
                504 => Self::GatewayTimeout(value.into()),
                _ => Self::Generic(value.into()),
            },
            None => Self::Generic(value.into()),
        }
    }
}

#[derive(Debug, Default)]
pub struct GenericApiErrorParams {
    pub error_code: i64,
    pub description: Option<String>,
    pub parameters: Option<TgApiErrorParams>,
}

impl<T> From<ApiResponse<T>> for GenericApiErrorParams {
    fn from(value: ApiResponse<T>) -> Self {
        Self {
            error_code: value.error_code.unwrap_or_default(),
            description: Some(
                value
                    .description
                    .unwrap_or_else(|| "No decsription".to_owned()),
            ),
            parameters: value.parameters,
        }
    }
}

impl Display for GenericApiErrorParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[API Error {}] {}",
            self.error_code,
            self.description
                .as_ref()
                .unwrap_or(&"No description".to_string())
        ))
    }
}
