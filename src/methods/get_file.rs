use conogram_derives::Request;
use serde::Serialize;

use crate::entities::file::File;

/// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a [File](https://core.telegram.org/bots/api/#file) object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [getFile](https://core.telegram.org/bots/api/#getfile) again.
///
/// API Reference: [link](https://core.telegram.org/bots/api/#getfile)
#[derive(Debug, Clone, Serialize, Request)]
#[conogram(result = File)]
pub struct GetFileParams {
    /// File identifier to get information about
    pub file_id: String,
}

// Divider: all content below this line will be preserved after code regen
