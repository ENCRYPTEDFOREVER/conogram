use serde::Serialize;

use crate::{
    entities::{input_rich_block::InputRichBlock, input_rich_message_media::InputRichMessageMedia},
    utils::deserialize_utils::is_false,
};

/// Describes a rich message to be sent. Exactly **one** of the fields *html*, *markdown*, or *blocks* must be used.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichmessage)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct InputRichMessage {
    /// *Optional*. Content of the rich message to send described as a list of blocks
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<InputRichBlock>,

    /// *Optional*. Content of the rich message to send described using HTML formatting. See [rich message formatting options](https://core.telegram.org/bots/api/#rich-message-formatting-options) for more details. Use *media* field to specify the media used in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,

    /// *Optional*. Content of the rich message to send described using Markdown formatting. See [rich message formatting options](https://core.telegram.org/bots/api/#rich-message-formatting-options) for more details. Use *media* field to specify the media used in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,

    /// *Optional*. List of media that are specified in the *markdown* or *html* fields using `tg://photo?id=`, `tg://video?id=`, and `tg://audio?id=` links
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<InputRichMessageMedia>,

    /// *Optional*. Pass *True* if the rich message must be shown right-to-left
    #[serde(skip_serializing_if = "is_false")]
    pub is_rtl: bool,

    /// *Optional*. Pass *True* to skip automatic detection of entities (e.g., URLs, email addresses, username mentions, hashtags, cashtags, bot commands, or phone numbers) in the text
    #[serde(skip_serializing_if = "is_false")]
    pub skip_entity_detection: bool,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichMessage {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        let mut form = form;
        for block in &self.blocks {
            form = block.form(form).await?;
        }
        Ok(form)
    }
}

impl InputRichMessage {
    /// Content of the rich message to send described as a list of blocks
    pub fn from_blocks(
        blocks: impl IntoIterator<Item = impl Into<InputRichBlock>>,
        is_rtl: bool,
        skip_entity_detection: bool,
    ) -> Self {
        Self {
            blocks: blocks.into_iter().map(Into::into).collect(),
            is_rtl,
            skip_entity_detection,
            ..Default::default()
        }
    }

    /// Content of the rich message to send described using HTML formatting. See [rich message formatting options](https://core.telegram.org/bots/api/#rich-message-formatting-options) for more details. Use *media* field to specify the media used in the message.
    pub fn from_html(
        html: impl Into<String>,
        media: impl IntoIterator<Item = impl Into<InputRichMessageMedia>>,
        is_rtl: bool,
        skip_entity_detection: bool,
    ) -> Self {
        Self {
            html: Some(html.into()),
            media: media.into_iter().map(Into::into).collect(),
            is_rtl,
            skip_entity_detection,
            ..Default::default()
        }
    }

    /// Content of the rich message to send described using Markdown formatting. See [rich message formatting options](https://core.telegram.org/bots/api/#rich-message-formatting-options) for more details. Use *media* field to specify the media used in the message.
    pub fn from_markdown(
        markdown: impl Into<String>,
        media: impl IntoIterator<Item = impl Into<InputRichMessageMedia>>,
        is_rtl: bool,
        skip_entity_detection: bool,
    ) -> Self {
        Self {
            markdown: Some(markdown.into()),
            media: media.into_iter().map(Into::into).collect(),
            is_rtl,
            skip_entity_detection,
            ..Default::default()
        }
    }
}
