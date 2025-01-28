#![allow(dead_code, unused_variables)]

use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};

use reqwest::{multipart::Form, Client, RequestBuilder, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::{
    api::ApiConfig,
    entities::misc::input_file::{GetFiles, InputFile},
    errors::{ConogramError, ConogramErrorType, TgApiError, TgApiErrorParams},
};

#[derive(Deserialize, Debug)]
pub(crate) struct ApiResponse<ReturnValue> {
    pub ok: bool,

    pub error_code: Option<i64>,
    pub description: Option<String>,
    pub parameters: Option<TgApiErrorParams>,

    pub result: Option<ReturnValue>,
}

pub(crate) struct ApiClient {
    base_url: String,
    http_client: Client,
    bot_config: ApiConfig,

    default_request_params: HashMap<String, HashMap<String, Value>>,
}

impl ApiClient {
    pub fn new(config: ApiConfig) -> Self {
        Self {
            base_url: if config.server_config.use_test_env {
                format!(
                    "{url}/bot{token}/test",
                    url = config.server_config.url,
                    token = config.token.leak()
                )
            } else {
                format!(
                    "{url}/bot{token}",
                    url = config.server_config.url,
                    token = config.token.leak()
                )
            },
            http_client: Client::new(),
            bot_config: config,
            default_request_params: HashMap::new(),
        }
    }

    /// # Errors
    /// Fails on ``value`` serialization fail
    pub fn set_default_request_param(
        &mut self,
        method: impl Into<String>,
        param_name: impl Into<String>,
        value: impl Serialize,
    ) -> Result<(), ConogramErrorType> {
        let method_entry = self
            .default_request_params
            .entry(method.into())
            .or_default();

        match method_entry.entry(param_name.into()) {
            Entry::Occupied(mut v) => {
                v.insert(serde_json::to_value(value)?);
            }
            Entry::Vacant(v) => {
                v.insert(serde_json::to_value(value)?);
            }
        }

        Ok(())
    }

    fn build_url(&self, method: &str) -> Url {
        reqwest::Url::from_str(&format!(
            "{base_url}/{method}",
            base_url = self.base_url,
            method = method
        ))
        .unwrap()
    }

    fn apply_default_params(&self, method: &str, default_value: &mut Value) {
        if let Some(method_entry) = self.default_request_params.get(method) {
            if let Value::Object(object) = default_value {
                for (param_name, v) in method_entry {
                    if !object.contains_key(param_name) {
                        log::debug!("Setting {param_name}={v} in {method}");
                        object.insert(param_name.clone(), v.clone());
                    }
                }
            }
        }
    }

    fn value_to_string(value: &Value) -> String {
        match value {
            Value::Null => "None".into(),
            Value::Bool(v) => format!("{v}"),
            Value::Number(v) => format!("{v}"),
            Value::String(v) => format!("\"{v}\""),
            Value::Array(v) => format!(
                "[{}]",
                v.iter()
                    .map(Self::value_to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Object(v) => format!(
                "{{{}}}",
                v.iter()
                    .map(|(key, v)| format!("\"{key}\":{}", Self::value_to_string(v)))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    fn process_api_response<ReturnType>(
        response: ApiResponse<ReturnType>,
    ) -> Result<ReturnType, ConogramErrorType> {
        Into::<Result<ReturnType, ConogramErrorType>>::into(response)
    }

    pub async fn method<
        ReturnType: DeserializeOwned + std::fmt::Debug,
        Params: Serialize + Sync + std::fmt::Debug,
    >(
        method: &str,
        builder: RequestBuilder,
        params: Option<&Params>,
    ) -> Result<ReturnType, ConogramError> {
        let response = match builder.send().await {
            Ok(r) => r,
            Err(err) => return Err(ConogramError::new(method, params, err.into())),
        };

        let response_text = match response.text().await {
            Ok(rt) => rt,
            Err(err) => return Err(ConogramError::new(method, params, err.into())),
        };

        let api_response = match serde_json::from_str::<ApiResponse<ReturnType>>(&response_text) {
            Ok(r) => r,
            Err(err) => return Err(ConogramError::new(method, params, err.into())),
        };

        match Self::process_api_response(api_response) {
            Ok(r) => Ok(r),
            Err(err) => {
                if let ConogramErrorType::ApiError(api_err) = &err {
                    match api_err {
                        TgApiError::RetryAfter(params) => {
                            if let Some(retry_after) =
                                params.parameters.as_ref().and_then(|p| p.retry_after)
                            {
                                if retry_after < 0 {
                                    log::warn!("[{method}] retry_after is < 0: {retry_after}");
                                } else if retry_after > 300 {
                                    log::warn!(
                                        "[{method}] retry_after is unusually big: {retry_after}"
                                    );
                                }
                            } else {
                                log::warn!(
                                    "[{method}] Got RetryAfter error but retry_after field is absent"
                                );
                            }
                        }
                        TgApiError::BadGateway(_) => log::warn!("[{method}] Got BadGateway"),
                        TgApiError::GatewayTimeout(_) => {
                            log::warn!("[{method}] Got GatewayTimeout");
                        }
                        _ => {}
                    }
                }

                Err(ConogramError::new(method, params, err))
            }
        }
    }

    pub async fn method_json<
        ReturnType: DeserializeOwned + std::fmt::Debug,
        Params: Serialize + Sync + std::fmt::Debug,
    >(
        &self,
        method: &str,
        params: Option<&Params>,
    ) -> Result<ReturnType, ConogramError> {
        let builder = match params {
            Some(params) => {
                let mut value: Value = match serde_json::to_value(params) {
                    Ok(v) => v,
                    Err(err) => return Err(ConogramError::new(method, params, err.into())),
                };
                self.apply_default_params(method, &mut value);

                log::debug!("Calling {method}({})", Self::value_to_string(&value));

                self.http_client.post(self.build_url(method)).json(&value)
            }
            None => self.http_client.post(self.build_url(method)),
        };

        Self::method(method, builder, params).await
    }

    pub async fn method_multipart_form<
        ReturnType: DeserializeOwned + std::fmt::Debug,
        Params: Serialize + GetFiles + Sync + std::fmt::Debug,
    >(
        &self,
        method: &str,
        params: Option<&Params>,
    ) -> Result<ReturnType, ConogramError> {
        let builder = match params {
            Some(params) => {
                let mut json_struct: Value = match serde_json::to_value(params) {
                    Ok(v) => v,
                    Err(err) => return Err(ConogramError::new(method, params, err.into())),
                };
                self.apply_default_params(method, &mut json_struct);

                log::debug!("Calling {method}({})", Self::value_to_string(&json_struct));

                let mut form = Form::new();
                if let Some(v) = json_struct.as_object() {
                    for (key, value) in v {
                        let value = match value {
                            Value::String(val) => val.to_string(),
                            other => other.to_string(),
                        };
                        form = form.text(key.clone(), value);
                    }
                }

                for file in params.get_files() {
                    match file {
                        InputFile::File(f) => {
                            let part = match f.get_part().await {
                                Ok(part) => part,
                                Err(err) => {
                                    return Err(ConogramError::new(method, params, err.into()))
                                }
                            };

                            form = form.part(f.get_uuid_str(), part);
                        }
                        InputFile::InMemory(f) => {
                            form = form.part(f.get_uuid_str(), f.get_part());
                        }
                        InputFile::FileIdOrURL(_) => {}
                    }
                }

                self.http_client
                    .post(self.build_url(method))
                    .multipart(form)
            }
            None => self.http_client.post(self.build_url(method)),
        };

        Self::method(method, builder, params).await
    }
}
