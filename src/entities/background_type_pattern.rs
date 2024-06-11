use crate::entities::background_fill::BackgroundFill;
use crate::entities::document::Document;
use crate::utils::deserialize_utils::is_false;
use serde::{Deserialize, Serialize};

///The background is a PNG or TGV (gzipped subset of SVG with MIME type “application/x-tgwallpattern”) pattern to be combined with the background fill chosen by the user.
///API Reference: [link](https://core.telegram.org/bots/api/#backgroundtypepattern)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BackgroundTypePattern {
    ///Document with the pattern
    pub document: Document,

    ///The background fill that is combined with the pattern
    pub fill: BackgroundFill,

    ///Intensity of the pattern when it is shown above the filled background; 0-100
    pub intensity: i64,

    ///*Optional*. *True*, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_inverted: bool,

    ///*Optional*. *True*, if the background moves slightly when the device is tilted
    #[serde(default, skip_serializing_if = "is_false")]
    pub is_moving: bool,
}
// Divider: all content below this line will be preserved after code regen
