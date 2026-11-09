use dashmap::DashMap;
use serde::{Deserialize, Serialize};

use crate::utils::sha2_digest;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CallbackDataCache {
    #[serde(flatten)]
    inner: DashMap<String, String>,
}

impl CallbackDataCache {
    pub fn put(&self, data: &str) -> Option<String> {
        if data.len() < 64 {
            None
        } else {
            let hash = sha2_digest(data);
            self.inner.insert(hash.clone(), data.to_owned());
            Some(hash)
        }
    }

    pub fn pull(&self, hash: &str) -> Option<String> {
        self.inner.get(hash).map(|data| data.clone())
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
