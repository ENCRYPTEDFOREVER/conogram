use serde::{Deserialize, Serialize};

use crate::entities::{document::Document, rich_block_caption::RichBlockCaption};

/// A block with a general file, corresponding to the custom HTML tag `<tg-document>`.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#richblockdocument)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "document", tag = "type")]
pub struct RichBlockDocument {
    /// The document
    pub document: Document,

    /// *Optional*. Caption of the block
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

// Divider: all content below this line will be preserved after code regen
