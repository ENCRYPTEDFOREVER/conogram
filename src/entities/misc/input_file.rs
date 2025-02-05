use std::{borrow::Cow, io, path::PathBuf};

use reqwest::multipart::Part;
use serde::Serialize;
use tokio::fs::File;
use uuid::Uuid;

///This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
///API Reference: [link](https://core.telegram.org/bots/api/#inputfile)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputFile {
    Local(LocalFile),
    FileIdOrUrl(String),
    InMemory(InMemoryFile),
}

type FileContents = Cow<'static, [u8]>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryFile {
    name: String,
    contents: FileContents,
    uuid: Uuid,
}

impl InMemoryFile {
    pub fn new(name: impl Into<String>, data: impl Into<FileContents>) -> Self {
        Self {
            name: name.into(),
            contents: data.into(),
            uuid: Uuid::new_v4(),
        }
    }

    #[must_use]
    pub fn get_attach_name(&self) -> String {
        format!("attach://{}", self.uuid)
    }

    #[must_use]
    pub fn get_uuid_str(&self) -> String {
        self.uuid.to_string()
    }

    #[must_use]
    pub fn get_form_part(&self) -> Part {
        Part::bytes(self.contents.clone()).file_name(self.name.clone())
    }

    #[must_use]
    pub fn into_form_part(self) -> Part {
        Part::bytes(self.contents).file_name(self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalFile {
    name: Option<String>,
    path: PathBuf,
    uuid: Uuid,
}

impl LocalFile {
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self {
            name: None,
            path: path.into(),
            uuid: Uuid::new_v4(),
        }
    }

    pub fn from_path_with_name(path: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            path: path.into(),
            uuid: Uuid::new_v4(),
        }
    }

    #[must_use]
    pub fn get_uuid_str(&self) -> String {
        self.uuid.to_string()
    }

    #[must_use]
    pub fn get_attach_name(&self) -> String {
        format!("attach://{}", self.uuid)
    }

    pub async fn get_form_part(&self) -> Result<Part, io::Error> {
        Ok(Part::stream(self.open().await?).file_name(self.get_name()))
    }

    pub async fn open(&self) -> Result<File, std::io::Error> {
        File::open(&self.path).await
    }

    #[must_use]
    pub fn get_name(&self) -> String {
        if let Some(filename) = &self.name {
            filename.clone()
        } else {
            match self.path.file_name() {
                Some(file_name) => (*file_name).to_string_lossy().to_string(),
                None => "unknown".to_string(),
            }
        }
    }
}

impl Default for InputFile {
    fn default() -> Self {
        Self::from_file_id("")
    }
}

impl InputFile {
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self::Local(LocalFile::from_path(path))
    }

    pub fn from_path_with_name(path: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        Self::Local(LocalFile::from_path_with_name(path, name))
    }

    pub fn from_file_id(file_id: impl Into<String>) -> Self {
        Self::FileIdOrUrl(file_id.into())
    }

    pub fn from_url(url: impl Into<String>) -> Self {
        Self::FileIdOrUrl(url.into())
    }

    pub fn from_data(name: impl Into<String>, data: impl Into<FileContents>) -> Self {
        Self::InMemory(InMemoryFile::new(name, data))
    }

    #[must_use]
    pub fn get_uuid(&self) -> Option<String> {
        match self {
            Self::Local(f) => Some(f.get_uuid_str()),
            _ => None,
        }
    }
}

impl<IntoString> From<IntoString> for InputFile
where
    IntoString: Into<String>,
{
    fn from(value: IntoString) -> Self {
        Self::FileIdOrUrl(value.into())
    }
}

pub trait GetFiles: Sized {
    fn get_files(&self) -> Vec<&InputFile>;
}

impl GetFiles for InputFile {
    fn get_files(&self) -> Vec<&InputFile> {
        vec![self]
    }
}
impl<T: GetFiles> GetFiles for Vec<T> {
    fn get_files(&self) -> Vec<&InputFile> {
        self.iter().flat_map(GetFiles::get_files).collect()
    }
}
impl<T: GetFiles> GetFiles for Option<T> {
    fn get_files(&self) -> Vec<&InputFile> {
        match self {
            Some(v) => v.get_files(),
            None => vec![],
        }
    }
}

impl Serialize for InputFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Local(f) => serializer.serialize_str(&f.get_attach_name()),
            Self::InMemory(f) => serializer.serialize_str(&f.get_attach_name()),
            Self::FileIdOrUrl(s) => serializer.serialize_str(s),
        }
    }
}
