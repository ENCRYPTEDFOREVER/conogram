use std::{io, path::PathBuf};

use reqwest::multipart::{Form, Part};
use serde::Serialize;
use tokio::fs::File;
use uuid::Uuid;

use crate::entities::{
    input_media_animation::InputMediaAnimation, input_media_audio::InputMediaAudio,
    input_media_document::InputMediaDocument, input_media_photo::InputMediaPhoto,
    input_media_video::InputMediaVideo, input_paid_media_photo::InputPaidMediaPhoto,
    input_paid_media_video::InputPaidMediaVideo,
};

/// This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
///
/// More info: <https://core.telegram.org/bots/api#sending-files>
///
/// API Reference: [link](https://core.telegram.org/bots/api/#inputfile)
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum InputFile {
    /// **Sending a newly uploaded file**
    ///
    /// Post the file using multipart/form-data in the usual way that files are uploaded via the browser. 10 MB max size for photos, 50 MB or 2 GB (with local Bot API server) for other files.
    Local(LocalFile),

    /// **Sending by file\_id**   
    /// *   It is not possible to change the file type when resending by **file\_id**. I.e. a [video](#video) can't be [sent as a photo](#sendphoto), a [photo](#photosize) can't be [sent as a document](#senddocument), etc.
    /// *   It is not possible to resend thumbnails.
    /// *   Resending a photo by **file\_id** will send all of its [sizes](#photosize).
    /// *   **file\_id** is unique for each individual bot and **can't** be transferred from one bot to another.
    /// *   **file\_id** uniquely identifies a file, but a file can have different valid **file\_id**s even for the same bot.
    FileId(String),

    /// **Sending by URL**
    /// *   When sending by URL the target file must have the correct MIME type (e.g., audio/mpeg for [sendAudio](#sendaudio), etc.).
    /// *   In [sendDocument](#senddocument), sending by URL will currently only work for **.PDF** and **.ZIP** files.
    /// *   To use [sendVoice](#sendvoice), the file must have the type audio/ogg and be no more than 1MB in size. 1-20MB voice notes will be sent as files.
    /// *   Other configurations may work but we can't guarantee that they will.
    Url(String),
}

impl InputFile {
    #[must_use]
    pub fn to_document(self) -> InputMediaDocument {
        self.into()
    }
    #[must_use]
    pub fn to_photo(self) -> InputMediaPhoto {
        self.into()
    }
    #[must_use]
    pub fn to_video(self) -> InputMediaVideo {
        self.into()
    }
    #[must_use]
    pub fn to_audio(self) -> InputMediaAudio {
        self.into()
    }
    #[must_use]
    pub fn to_animation(self) -> InputMediaAnimation {
        self.into()
    }
    #[must_use]
    pub fn to_paid_photo(self) -> InputPaidMediaPhoto {
        self.into()
    }
    #[must_use]
    pub fn to_paid_vide(self) -> InputPaidMediaVideo {
        self.into()
    }
}

// TODO: allow borrowing
type FileContents = Vec<u8>;
#[derive(Debug, PartialEq, Eq)]
pub struct LocalFile {
    name: Option<String>,
    data: Option<FileContents>,
    path: Option<PathBuf>,
    uuid: Uuid,
}

impl LocalFile {
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self {
            name: None,
            path: Some(path.into()),
            uuid: Uuid::new_v4(),

            data: None,
        }
    }

    pub fn from_path_with_name(path: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            path: Some(path.into()),
            uuid: Uuid::new_v4(),
            data: None,
        }
    }

    pub fn from_data(name: impl Into<String>, data: impl Into<FileContents>) -> Self {
        Self {
            name: Some(name.into()),
            data: Some(data.into()),
            uuid: Uuid::new_v4(),
            path: None,
        }
    }

    #[must_use]
    pub fn get_attach_name(&self) -> String {
        self.uuid.to_string()
    }

    /// ## Panics
    ///
    /// If `path` and `data` are None
    pub async fn get_form_part(&self) -> Result<Part, io::Error> {
        let file_name = self.get_name();
        if let Some(path) = &self.path {
            let file = File::open(path).await?;
            Ok(Part::stream(file).file_name(file_name))
        } else if let Some(data) = &self.data {
            Ok(Part::bytes(data.clone()).file_name(file_name))
        } else {
            panic!("path or data must be set for LocalFile",)
        }
    }

    #[must_use]
    pub fn get_name(&self) -> String {
        if let Some(name) = &self.name {
            name.clone()
        } else if let Some(path) = self.path.as_ref().and_then(|p| p.file_name()) {
            path.to_string_lossy().to_string()
        } else {
            "unknown.ext".to_string()
        }
    }
}

impl Default for InputFile {
    fn default() -> Self {
        Self::from_file_id("")
    }
}

impl InputFile {
    /// Upload a new file located by ``path``
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self::Local(LocalFile::from_path(path))
    }

    /// Upload a new file located by ``path``
    pub fn from_path_with_name(path: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        Self::Local(LocalFile::from_path_with_name(path, name))
    }

    /// **Sending by file\_id**   
    /// *   It is not possible to change the file type when resending by **file\_id**. I.e. a [video](#video) can't be [sent as a photo](#sendphoto), a [photo](#photosize) can't be [sent as a document](#senddocument), etc.
    /// *   It is not possible to resend thumbnails.
    /// *   Resending a photo by **file\_id** will send all of its [sizes](#photosize).
    /// *   **file\_id** is unique for each individual bot and **can't** be transferred from one bot to another.
    /// *   **file\_id** uniquely identifies a file, but a file can have different valid **file\_id**s even for the same bot.
    pub fn from_file_id(file_id: impl Into<String>) -> Self {
        Self::FileId(file_id.into())
    }

    /// **Sending by URL**
    /// *   When sending by URL the target file must have the correct MIME type (e.g., audio/mpeg for [sendAudio](#sendaudio), etc.).
    /// *   In [sendDocument](#senddocument), sending by URL will currently only work for **.PDF** and **.ZIP** files.
    /// *   To use [sendVoice](#sendvoice), the file must have the type audio/ogg and be no more than 1MB in size. 1-20MB voice notes will be sent as files.
    /// *   Other configurations may work but we can't guarantee that they will.
    pub fn from_url(url: impl Into<String>) -> Self {
        Self::Url(url.into())
    }

    /// Upload a new file from RAM
    pub fn from_data(name: impl Into<String>, data: impl Into<FileContents>) -> Self {
        Self::Local(LocalFile::from_data(name, data))
    }
}

impl<IntoString> From<IntoString> for InputFile
where
    IntoString: Into<String>,
{
    fn from(value: IntoString) -> Self {
        Self::Url(value.into())
    }
}

pub trait GetFiles {
    fn form(
        &self,
        form: Form,
    ) -> impl std::future::Future<Output = Result<Form, std::io::Error>> + Send;
}

impl GetFiles for LocalFile {
    async fn form(&self, form: Form) -> Result<Form, std::io::Error> {
        Ok(form.part(self.get_attach_name(), self.get_form_part().await?))
    }
}

impl GetFiles for InputFile {
    async fn form(&self, form: Form) -> Result<Form, std::io::Error> {
        if let Self::Local(local_file) = self {
            local_file.form(form).await
        } else {
            Ok(form)
        }
    }
}

impl<T: GetFiles + Sync> GetFiles for Option<T> {
    async fn form(&self, form: Form) -> Result<Form, std::io::Error> {
        if let Some(input_file) = self {
            input_file.form(form).await
        } else {
            Ok(form)
        }
    }
}

impl<T: GetFiles + Sync> GetFiles for Vec<T> {
    async fn form(&self, form: Form) -> Result<Form, std::io::Error> {
        let mut form = form;
        for input_file in self {
            form = input_file.form(form).await?;
        }
        Ok(form)
    }
}

// impl GetFiles for Vec<InputFile> {
//     async fn form(&self, form: Form) -> Result<Form, std::io::Error> {
//         let mut form = form;
//         for input_file in self {
//             form = input_file.form(form).await?;
//         }
//         Ok(form)
//     }
// }

// pub trait GetFiles: Sized {
//     fn get_files(&self) -> Vec<&InputFile>;
// }
// impl GetFiles for LocalFile {
//     fn get_files(&self) -> Vec<&InputFile> {
//         vec![self]
//     }
// }
// impl GetFiles for InputFile {
//     fn get_files(&self) -> Vec<&InputFile> {
//         vec![self]
//     }
// }
// impl<T: GetFiles> GetFiles for Vec<T> {
//     fn get_files(&self) -> Vec<&InputFile> {
//         self.iter().flat_map(GetFiles::get_files).collect()
//     }
// }
// impl<T: GetFiles> GetFiles for Option<T> {
//     fn get_files(&self) -> Vec<&InputFile> {
//         match self {
//             Some(v) => v.get_files(),
//             None => vec![],
//         }
//     }
// }

impl Serialize for LocalFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("attach://{}", self.get_attach_name()))
    }
}

impl Clone for LocalFile {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            data: self.data.clone(),
            path: self.path.clone(),
            uuid: Uuid::new_v4(),
        }
    }
}
