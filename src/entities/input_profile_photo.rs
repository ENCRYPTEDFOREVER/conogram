use serde::Serialize;

use crate::entities::{
    input_profile_photo_animated::InputProfilePhotoAnimated,
    input_profile_photo_static::InputProfilePhotoStatic, misc::input_file::GetFiles,
};

/// This object describes a profile photo to set. Currently, it can be one of
///
/// * [InputProfilePhotoStatic](https://core.telegram.org/bots/api/#inputprofilephotostatic)
/// * [InputProfilePhotoAnimated](https://core.telegram.org/bots/api/#inputprofilephotoanimated)
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputprofilephoto)
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum InputProfilePhoto {
    /// A static profile photo in the .JPG format.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputprofilephotostatic)
    #[serde(rename = "static")]
    Static(InputProfilePhotoStatic),

    /// An animated profile photo in the MPEG4 format.
    ///
    /// API Reference: [link](https://core.telegram.org/bots/api/#inputprofilephotoanimated)
    #[serde(rename = "animated")]
    Animated(InputProfilePhotoAnimated),
}

impl Default for InputProfilePhoto {
    fn default() -> Self {
        Self::Static(InputProfilePhotoStatic::default())
    }
}

impl From<InputProfilePhotoStatic> for InputProfilePhoto {
    fn from(value: InputProfilePhotoStatic) -> Self {
        Self::Static(value)
    }
}

impl From<InputProfilePhotoAnimated> for InputProfilePhoto {
    fn from(value: InputProfilePhotoAnimated) -> Self {
        Self::Animated(value)
    }
}

impl GetFiles for InputProfilePhoto {
    async fn form(
        &self,
        form: reqwest::multipart::Form,
    ) -> Result<reqwest::multipart::Form, std::io::Error> {
        match self {
            Self::Animated(m) => m.form(form).await,
            Self::Static(m) => m.form(form).await,
        }
    }
}
// Divider: all content below this line will be preserved after code regen
