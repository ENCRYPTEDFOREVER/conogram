use crate::api::API;
use crate::entities::file::File;
use crate::errors::ConogramError;
use crate::impl_into_future;
use crate::request::RequestT;
use serde::Serialize;
use std::future::{Future, IntoFuture};
use std::pin::Pin;

#[derive(Debug, Clone, Serialize)]
pub struct GetFileParams {
    pub file_id: String,
}

impl_into_future!(GetFileRequest<'a>);

///Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a [File](https://core.telegram.org/bots/api/#file) object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api/#getfile) again.
#[derive(Clone)]
pub struct GetFileRequest<'a> {
    api: &'a API,
    params: GetFileParams,
}

impl<'a> RequestT for GetFileRequest<'a> {
    type ParamsType = GetFileParams;
    type ReturnType = File;
    fn get_name() -> &'static str {
        "getFile"
    }
    fn get_api_ref(&self) -> &API {
        self.api
    }
    fn get_params_ref(&self) -> &Self::ParamsType {
        &self.params
    }
    fn is_multipart() -> bool {
        false
    }
}
impl<'a> GetFileRequest<'a> {
    pub fn new(api: &'a API, file_id: impl Into<String>) -> Self {
        Self {
            api,
            params: GetFileParams {
                file_id: file_id.into(),
            },
        }
    }

    ///File identifier to get information about
    #[must_use]
    pub fn file_id(mut self, file_id: impl Into<String>) -> Self {
        self.params.file_id = file_id.into();
        self
    }
}

impl API {
    ///Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a [File](https://core.telegram.org/bots/api/#file) object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api/#getfile) again.
    pub fn get_file(&self, file_id: impl Into<String>) -> GetFileRequest {
        GetFileRequest::new(self, file_id)
    }
}

// Divider: all content below this line will be preserved after code regen
