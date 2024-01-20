use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///Describes the options used for link preview generation.
///API Reference: [link](https://core.telegram.org/bots/api/#linkpreviewoptions)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    ///*Optional*. *True*, if the link preview is disabled
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_disabled: bool,

    ///*Optional*. URL to use for the link preview. If empty, then the first URL found in the message text will be used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    ///*Optional*. *True*, if the media in the link preview is suppposed to be shrunk; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(default, skip_serializing_if = "is_false")]
    pub prefer_small_media: bool,

    ///*Optional*. *True*, if the media in the link preview is suppposed to be enlarged; ignored if the URL isn't explicitly specified or media size change isn't supported for the preview
    #[serde(default, skip_serializing_if = "is_false")]
    pub prefer_large_media: bool,

    ///*Optional*. *True*, if the link preview must be shown above the message text; otherwise, the link preview will be shown below the message text
    #[serde(default, skip_serializing_if = "is_false")]
    pub show_above_text: bool,
}
// Divider: all content below this line will be preserved after code regen
impl LinkPreviewOptions {
    pub fn disabled() -> Self {
        Self {
            is_disabled: true,
            ..Default::default()
        }
    }

    pub fn new(
        url: Option<impl Into<String>>,
        prefer_small_media: bool,
        prefer_large_media: bool,
        show_above_text: bool,
    ) -> Self {
        Self {
            is_disabled: false,
            url: url.map(Into::into),
            prefer_small_media,
            prefer_large_media,
            show_above_text,
        }
    }
}
