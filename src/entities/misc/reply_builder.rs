use std::range::Range;

use crate::{
    api::Api,
    entities::{
        message::{InputMessageText, Message},
        misc::chat_id::ChatId,
        reply_parameters::ReplyParameters,
    },
    methods::send_message::SendMessageRequest,
};

#[derive(Debug)]
pub struct ReplyBuilder<'a> {
    pub(crate) api: &'a Api,
    pub(crate) reply_parameters: ReplyParameters,
    pub(crate) chat_id: ChatId,
    pub(crate) ephemeral_target_id: Option<i64>,
    pub(crate) allow_ephemeral_leak: bool,
    pub(crate) message_thread_id: Option<i64>,

    message: &'a Message,
    original_sender_id: i64,
}

impl<'a> ReplyBuilder<'a> {
    pub fn reply(api: &'a Api, message: &'a Message) -> Self {
        let reply_parameters = ReplyParameters::reply(message);
        let ephemeral_target_id = if reply_parameters.ephemeral_message_id.is_some() {
            Some(message.from_id())
        } else {
            None
        };

        Self {
            api,
            message,
            reply_parameters,
            chat_id: message.chat.id.into(),
            original_sender_id: message.from_id(),
            ephemeral_target_id,
            allow_ephemeral_leak: true,
            message_thread_id: message.message_thread_id,
        }
    }

    #[must_use]
    pub fn quote(mut self, range: Option<impl Into<Range<usize>>>) -> Self {
        self.reply_parameters.add_quote(self.message, range);
        self
    }

    /// Reply to this message in another chat
    #[must_use]
    pub fn chat(mut self, chat_id: impl Into<ChatId>) -> Self {
        self.chat_id = chat_id.into();
        self.reply_parameters
            .set_original_chat(self.message.chat.id);
        self
    }

    /// See: https://core.telegram.org/bots/api/#ephemeral-messages-and-commands
    #[must_use]
    pub const fn ephemeral_disable(mut self) -> Self {
        self.ephemeral_target_id = None;
        self.allow_ephemeral_leak = true;
        self.reply_parameters.ephemeral_message_id = None;
        self.reply_parameters.allow_sending_without_reply = true;
        self
    }

    /// The message will be visible only for the original sender
    ///
    /// See: https://core.telegram.org/bots/api/#ephemeral-messages-and-commands
    #[must_use]
    pub const fn ephemeral(mut self) -> Self {
        self.ephemeral_target_id = Some(self.original_sender_id);
        self
    }

    /// The message will be visible only for the original sender and will fail to send if not possible to make it ephemeral
    /// By default ephemeral messages will send as regular messages if it's not possible to send as ephemeral (i.e. target is a channel, anonymous admin or a private chat)
    ///
    /// See: https://core.telegram.org/bots/api/#ephemeral-messages-and-commands
    #[must_use]
    pub const fn force_ephemeral(mut self) -> Self {
        self.ephemeral_target_id = Some(self.original_sender_id);
        self.allow_ephemeral_leak = false;
        self
    }

    /// The message will be visible only for the specified user
    ///
    /// See: https://core.telegram.org/bots/api/#ephemeral-messages-and-commands
    #[must_use]
    pub fn ephemeral_for(mut self, user_id: impl Into<i64>) -> Self {
        self.ephemeral_target_id = Some(user_id.into());
        self
    }

    /// The message will be visible only for the specified user and will not fallback to regular message
    ///
    /// See: https://core.telegram.org/bots/api/#ephemeral-messages-and-commands
    #[must_use]
    pub fn force_ephemeral_for(mut self, user_id: impl Into<i64>) -> Self {
        self.ephemeral_target_id = Some(user_id.into());
        self.allow_ephemeral_leak = false;
        self
    }

    /// Use this method to send text messages. On success, the sent [Message](https://core.telegram.org/bots/api/#message) is returned.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#sendmessage)
    pub fn text(self, text: impl Into<InputMessageText>) -> SendMessageRequest<'a> {
        self.message(text)
    }
}
