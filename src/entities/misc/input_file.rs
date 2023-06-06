use std::path::PathBuf;

use std::borrow::Cow;
use std::collections::HashMap;

use serde::Serialize;
use tokio::fs::File;
use uuid::Uuid;

///This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
///API Reference: [link](https://core.telegram.org/bots/api/#inputfile)
#[derive(Debug, Clone, PartialEq)]
pub enum InputFile {
    File(LocalFile),
    FileIdOrURL(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalFile {
    path: PathBuf,
    uuid: Uuid,
}

impl LocalFile {
    pub fn from_path(path: impl Into<String>) -> Self {
        Self {
            path: PathBuf::from(path.into()),
            uuid: Uuid::new_v4(),
        }
    }

    pub fn get_uuid_str(&self) -> String {
        self.uuid.to_string()
    }

    pub fn get_attach_name(&self) -> String {
        format!("attach://{}", self.uuid)
    }

    pub async fn open(&self) -> Result<File, std::io::Error> {
        File::open(&self.path).await
    }

    pub fn get_file_name(&self) -> String {
        match self.path.file_name() {
            Some(file_name) => (*file_name).to_string_lossy().to_string(),
            None => "unknown".to_string(),
        }
    }
}

impl Default for InputFile {
    fn default() -> Self {
        Self::from_file_id("")
    }
}

impl InputFile {
    pub fn from_path(path: impl Into<String>) -> Self {
        InputFile::File(LocalFile::from_path(path))
    }

    pub fn from_file_id(file_id: impl Into<String>) -> Self {
        InputFile::FileIdOrURL(file_id.into())
    }

    pub fn from_url(url: impl Into<String>) -> Self {
        InputFile::FileIdOrURL(url.into())
    }

    pub fn get_attach_name(&self) -> String {
        match self {
            InputFile::File(f) => f.get_attach_name(),
            InputFile::FileIdOrURL(id_or_url) => id_or_url.clone(),
        }
    }

    pub fn get_uuid(&self) -> Option<String> {
        match self {
            InputFile::File(f) => Some(f.get_uuid_str()),
            _ => None,
        }
    }
}

impl<IntoString> From<IntoString> for InputFile
where
    IntoString: Into<String>,
{
    fn from(value: IntoString) -> Self {
        Self::FileIdOrURL(value.into())
    }
}

/// If we have Cows why not have Moose
pub type Moose = Cow<'static, str>;

pub trait GetFiles {
    fn get_files(&self) -> HashMap<Moose, &InputFile>;
}

impl Serialize for InputFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            InputFile::File(f) => serializer.serialize_str(&f.get_attach_name()),
            InputFile::FileIdOrURL(s) => serializer.serialize_str(s),
        }
    }
}
