#![allow(dead_code)]

use std::fmt::Display;

use serde::Deserialize;

use crate::client::ApiResponse;

#[derive(Debug)]
pub enum Error {
    ApiError(TgApiError),
    RequestError(reqwest::Error),
    SerdeError(serde_json::Error),
    IO(std::io::Error),
    Custom(String),
    TokenValidationError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ApiError(err) => err.fmt(f),
            Error::RequestError(err) => err.fmt(f),
            Error::SerdeError(err) => err.fmt(f),
            Error::IO(err) => err.fmt(f),
            Error::TokenValidationError => f.write_str("API Token is malformed"),
            Error::Custom(err_msg) => err_msg.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl From<TgApiError> for Error {
    fn from(value: TgApiError) -> Self {
        Self::ApiError(value)
    }
}

impl<ReturnType> From<ApiResponse<ReturnType>> for Result<ReturnType, Error> {
    fn from(value: ApiResponse<ReturnType>) -> Self {
        match value.ok {
            true => Ok(value.result.unwrap()),
            false => Err(Error::ApiError(TgApiError::from(value))),
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct TgApiErrorParams {
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<i64>,
}

#[derive(Debug)]
pub enum TgApiError {
    Generic(GenericApiErrorParams),
    RetryAfter(i64),
    NotFound,
    Unauthorized,
    Conflict,
    BadGateway,
    GatewayTimeout,
}

impl Display for TgApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic(params) => f.write_fmt(format_args!(
                "[API Error {code}] {desc}",
                code = params.error_code,
                desc = params
                    .description
                    .as_ref()
                    .unwrap_or(&"No description".into())
            )),
            Self::RetryAfter(wait_for) => f.write_fmt(format_args!(
                "[API Error 429] Retry request after {wait_for}s",
            )),
            Self::Unauthorized => f.write_str("[API Error 401] Unauthorized: API token is incorrect"),
            Self::NotFound => f.write_str("[API Error 404] Not Found: most likely API token is incorrect"),
            Self::BadGateway => f.write_str("[API Error 502] Bad Gateway"),
            Self::GatewayTimeout => f.write_str("[API Error 504] Gateway Timeout"),
            Self::Conflict => f.write_str("[API Error 409] Conflict: terminated by other getUpdates request, make sure that only one bot instance is running"),
        }
    }
}

impl Default for TgApiError {
    fn default() -> Self {
        Self::Generic(GenericApiErrorParams::default())
    }
}

impl<ReturnType> From<ApiResponse<ReturnType>> for TgApiError {
    fn from(value: ApiResponse<ReturnType>) -> Self {
        match value.error_code {
            Some(error_code) => match error_code {
                401 => Self::Unauthorized,
                404 => Self::NotFound,
                409 => Self::Conflict,
                429 => Self::RetryAfter(
                    value
                        .parameters
                        .unwrap_or_default()
                        .retry_after
                        .unwrap_or_default(),
                ),
                502 => Self::BadGateway,
                504 => Self::GatewayTimeout,
                _ => Self::Generic(GenericApiErrorParams {
                    error_code,
                    description: value.description,
                    parameters: value.parameters,
                }),
            },
            None => Self::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct GenericApiErrorParams {
    pub error_code: i64,
    pub description: Option<String>,
    pub parameters: Option<TgApiErrorParams>,
}
