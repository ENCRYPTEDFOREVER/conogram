use std::fmt::Write as _;

use dashmap::DashMap;
use sha2::Digest;

#[derive(Debug, Default)]
pub struct CallbackDataCache {
    inner: DashMap<String, String>,
}

impl CallbackDataCache {
    pub fn put(&self, data: &str) -> Option<String> {
        if data.len() < 64 {
            None
        } else {
            let digest = sha2::Sha256::digest(data);
            let mut hash = String::with_capacity(64);
            for b in digest {
                let _ = write!(&mut hash, "{b:02x}");
            }
            self.inner.insert(hash.clone(), data.to_owned());
            Some(hash)
        }
    }

    pub fn pull(&self, hash: &str) -> Option<String> {
        self.inner.get(hash).map(|data| data.clone())
    }
}
