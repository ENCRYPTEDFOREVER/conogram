use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::Serialize;

use crate::{
    api::Api, entities::bot_command_scope::BotCommandScope, errors::ConogramError,
    impl_into_future, request::RequestT,
};

#[derive(Debug, Clone, Serialize)]
pub struct DeleteMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl_into_future!(DeleteMyCommandsRequest<'a>);

///Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, [higher level commands](https://core.telegram.org/bots/api/#determining-list-of-commands) will be shown to affected users. Returns *True* on success.
#[derive(Clone)]
pub struct DeleteMyCommandsRequest<'a> {
    api: &'a Api,
    params: DeleteMyCommandsParams,
}

impl RequestT for DeleteMyCommandsRequest<'_> {
    type ParamsType = DeleteMyCommandsParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "deleteMyCommands"
    }
    fn get_api_ref(&self) -> &Api {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        false
    }
}
impl<'a> DeleteMyCommandsRequest<'a> {
    pub fn new(api: &'a Api) -> Self {
        Self {
            api,
            params: DeleteMyCommandsParams {
                scope: Option::default(),
                language_code: Option::default(),
            },
        }
    }

    ///A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to [BotCommandScopeDefault](https://core.telegram.org/bots/api/#botcommandscopedefault).
    #[must_use]
    pub fn scope(mut self, scope: impl Into<BotCommandScope>) -> Self {
        self.params.scope = Some(scope.into());
        self
    }

    ///A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[must_use]
    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.params.language_code = Some(language_code.into());
        self
    }
}

impl Api {
    ///Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, [higher level commands](https://core.telegram.org/bots/api/#determining-list-of-commands) will be shown to affected users. Returns *True* on success.
    pub fn delete_my_commands(&self) -> DeleteMyCommandsRequest {
        DeleteMyCommandsRequest::new(self)
    }
}

// Divider: all content below this line will be preserved after code regen
