#![allow(dead_code, unused_variables)]

use reqwest::multipart::{Form, Part};
use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;

use crate::api::APIConfig;
use crate::entities::misc::input_file::{GetFiles, InputFile};
use crate::errors::{ConogramError, ConogramErrorType, TgApiErrorParams};

#[derive(Deserialize, Debug)]
pub(crate) struct ApiResponse<ReturnValue> {
    pub ok: bool,

    pub error_code: Option<i64>,
    pub description: Option<String>,
    pub parameters: Option<TgApiErrorParams>,

    pub result: Option<ReturnValue>,
}

pub struct ApiClient {
    base_url: String,
    http_client: Client,
    bot_config: APIConfig,

    default_params: HashMap<String, HashMap<String, Value>>,
}

impl ApiClient {
    pub fn new(config: APIConfig) -> ApiClient {
        Self {
            base_url: match config.server_config.use_test_env {
                false => format!(
                    "{url}:{port}/bot{token}",
                    url = config.server_config.url,
                    port = config.server_config.port,
                    token = config.token
                ),
                true => format!(
                    "{url}:{port}/bot{token}/test",
                    url = config.server_config.url,
                    port = config.server_config.port,
                    token = config.token
                ),
            },
            http_client: Client::new(),
            bot_config: config,
            default_params: HashMap::new(),
        }
    }

    pub fn set_default_request_param(
        &mut self,
        method: impl Into<String>,
        param_name: impl Into<String>,
        value: impl Serialize,
    ) -> Result<(), ConogramErrorType> {
        let method_entry = self
            .default_params
            .entry(method.into())
            .or_insert_with(HashMap::new);

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

    fn apply_default_params(&self, method: &str, value: &mut Value) {
        if let Some(method_entry) = self.default_params.get(method) {
            if let Value::Object(object) = value {
                for (param_name, v) in method_entry.iter() {
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
        Params: Serialize + std::fmt::Debug,
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
            Err(err) => return Err(ConogramError::new(method, params, err.into())),
        }
    }

    pub async fn method_json<
        ReturnType: DeserializeOwned + std::fmt::Debug,
        Params: Serialize + std::fmt::Debug,
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
        Params: Serialize + GetFiles + std::fmt::Debug,
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
                for (key, value) in json_struct.as_object().unwrap() {
                    let value = match value {
                        Value::String(val) => val.to_string(),
                        other => other.to_string(),
                    };
                    form = form.text(key.clone(), value);
                }

                for (key, file) in params.get_files().into_iter() {
                    match file {
                        InputFile::File(f) => {
                            let file_name = f.get_file_name();
                            let file = match f.open().await {
                                Ok(f) => f,
                                Err(err) => {
                                    return Err(ConogramError::new(method, params, err.into()))
                                }
                            };
                            let part = Part::stream(file).file_name(file_name);

                            if key == "media" {
                                form = form.part(f.get_uuid_str(), part);
                            } else {
                                form = form.part(key, part);
                            }
                        }
                        InputFile::FileIdOrURL(value) => {
                            form = form.text(key.into_owned(), value.clone());
                        }
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
