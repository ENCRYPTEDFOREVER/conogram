use std::{fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    callback_data_cache::CallbackDataCache,
    chat_member_cache::ChatMemberCache,
    errors::{ConogramError, ConogramErrorT},
};

/// A structure which holds conogram caches. Drop impl saves this to [temporary directory](std::env::temp_dir) which by default on some systems is purged on reboot etc. You may want to use non-default temporary directory to avoid this
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Storage {
    #[serde(skip)]
    pub file_path: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_member_cache: Option<ChatMemberCache>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub callback_data_cache: Option<CallbackDataCache>,
}

impl Storage {
    pub(crate) fn load(key: impl Display) -> Result<Self, ConogramError> {
        let file_path = Self::get_file_path(key);
        if !file_path.exists() {
            Ok(Self {
                file_path,
                chat_member_cache: None,
                callback_data_cache: None,
            })
        } else {
            let method_name = "conogramLoadPersistentStorage";
            let json = std::fs::read_to_string(&file_path).with_details(method_name, &file_path)?;
            let mut v =
                serde_json::from_str::<Self>(&json).with_details(method_name, &file_path)?;
            log::info!("Loaded storage from \"{}\"", file_path.display());
            v.file_path = file_path;
            Ok(v)
        }
    }

    pub fn save(&self) -> Result<bool, ConogramError> {
        if self
            .callback_data_cache
            .as_ref()
            .is_none_or(CallbackDataCache::is_empty)
            && self
                .chat_member_cache
                .as_ref()
                .is_none_or(ChatMemberCache::is_empty)
        {
            return Ok(false);
        }

        let method_name = "conogramSavePersistentStorage";
        let json = serde_json::to_string(self).with_details(method_name, &self.file_path)?;
        if let Some(parent) = self.file_path.parent() {
            std::fs::create_dir_all(parent).with_details(method_name, &self.file_path)?;
        }

        let tmp_path = self.file_path.with_extension("tmp");
        std::fs::write(&tmp_path, json).with_details(method_name, &self.file_path)?;
        std::fs::rename(&tmp_path, &self.file_path).with_details(method_name, &self.file_path)?;
        log::info!("Saved storage to \"{}\"", self.file_path.display());
        Ok(true)
    }

    fn get_file_path(key: impl Display) -> PathBuf {
        let tmp_dir = std::env::temp_dir();
        let filename = format!("storage_{key}.json");
        tmp_dir.join(filename)
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        if let Err(err) = self.save() {
            log::error!("Failed to save storage to disk: {err}");
        }
    }
}
