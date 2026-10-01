use serde::Serialize;

use crate::entities::{
    input_media_document::InputMediaDocument, rich_block_caption::RichBlockCaption,
};

/// A block with a general file, corresponding to the custom HTML tag `<tg-document>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputrichblockdocument)
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename = "document", tag = "type")]
pub struct InputRichBlockDocument {
    /// The document. Caption is ignored.
    pub document: InputMediaDocument,

    /// *Optional*. Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen

use crate::entities::misc::input_file::GetFiles;

impl GetFiles for InputRichBlockDocument {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        self.document.form(form).await
    }
}
