use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::API,
    entities::{bot_command::BotCommand, bot_command_scope::BotCommandScope},
    errors::ConogramError,
    impl_into_future,
    request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl_into_future!(GetMyCommandsRequest<'a>);

///Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of [BotCommand](https://core.telegram.org/bots/api/#botcommand) objects. If commands aren't set, an empty list is returned.
#[derive(Clone)]
pub struct GetMyCommandsRequest<'a> {
    api: &'a API,
    params: GetMyCommandsParams,
}

impl<'a> RequestT for GetMyCommandsRequest<'a> {
    type ParamsType = GetMyCommandsParams;
    type ReturnType = Vec<BotCommand>;
    fn get_name() -> &'static str {
        "getMyCommands"
    }
    fn get_api_ref(&self) -> &API {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        false
    }
}
impl<'a> GetMyCommandsRequest<'a> {
    pub fn new(api: &'a API) -> Self {
        Self {
            api,
            params: GetMyCommandsParams {
                scope: Option::default(),
                language_code: Option::default(),
            },
        }
    }

    ///A JSON-serialized object, describing scope of users. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault).
    #[must_use]
    pub fn scope(mut self, scope: impl Into<BotCommandScope>) -> Self {
        self.params.scope = Some(scope.into());
        self
    }

    ///A two-letter ISO 639-1 language code or an empty string
    #[must_use]
    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.params.language_code = Some(language_code.into());
        self
    }
}

impl API {
    ///Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of [BotCommand](https://core.telegram.org/bots/api/#botcommand) objects. If commands aren't set, an empty list is returned.
    pub fn get_my_commands(&self) -> GetMyCommandsRequest {
        GetMyCommandsRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
