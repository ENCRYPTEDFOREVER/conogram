use crate::api::API;
use crate::entities::bot_command::BotCommand;
use crate::entities::bot_command_scope::BotCommandScope;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct SetMyCommandsParams {
    pub commands: Vec<BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl_into_future!(SetMyCommandsRequest<'a>);

///Use this method to change the list of the bot's commands. See [this manual](https://core.telegram.org/bots/features#commands) for more details about bot commands. Returns *True* on success.
#[derive(Clone)]
pub struct SetMyCommandsRequest<'a> {
    api: &'a API,
    params: SetMyCommandsParams,
}

impl<'a> RequestT for SetMyCommandsRequest<'a> {
    type ParamsType = SetMyCommandsParams;
    type ReturnType = bool;
    fn get_name() -> &'static str {
        "setMyCommands"
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
impl<'a> SetMyCommandsRequest<'a> {
    pub fn new(api: &'a API, commands: impl IntoIterator<Item = impl Into<BotCommand>>) -> Self {
        Self {
            api,
            params: SetMyCommandsParams {
                commands: commands.into_iter().map(Into::into).collect(),
                scope: Option::default(),
                language_code: Option::default(),
            },
        }
    }

    ///A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    #[must_use]
    pub fn commands(mut self, commands: impl IntoIterator<Item = impl Into<BotCommand>>) -> Self {
        self.params.commands = commands.into_iter().map(Into::into).collect();
        self
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

impl API {
    ///Use this method to change the list of the bot's commands. See [this manual](https://core.telegram.org/bots/features#commands) for more details about bot commands. Returns *True* on success.
    pub fn set_my_commands(
        &self,
        commands: impl IntoIterator<Item = impl Into<BotCommand>>,
    ) -> SetMyCommandsRequest {
        SetMyCommandsRequest::new(self, commands)
    }
}

// Divider: all content below this line will be preserved after code regen
