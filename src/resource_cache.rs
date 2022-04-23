use std::path::PathBuf;
use std::{collections::HashMap, rc::Rc};

use bytes::Bytes;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use lru::LruCache;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::error::BadPdfLayout;

const MAX_RESOURCE_COUNT: usize = 100;

pub struct ResourceCache {
    cache: LruCache<String, Rc<Bytes>>,
    client: Client,
    storage_cache: StorageCache,
}

#[derive(Deserialize, Serialize)]
struct FileInfo {
    #[serde(with = "ts_seconds")]
    timestamp: DateTime<Utc>,
    etag: String,
    cache_location: String,
}

#[derive(Deserialize, Serialize)]
struct StorageCache {
    files: HashMap<String, FileInfo>,
}

impl StorageCache {
    pub fn with_cache_index(path: &str) -> StorageCache {
        let mut cache_dir = PathBuf::from(path);
        cache_dir.pop();

        std::fs::create_dir_all(cache_dir).unwrap();

        let cache = std::fs::read_to_string(path);

        if let Ok(json) = cache {
            if let Ok(cache) = serde_json::from_str::<StorageCache>(&json) {
                return cache;
            }
        }

        StorageCache {
            files: HashMap::new(),
        }
    }
}

impl ResourceCache {
    pub fn new() -> Self {
        Self {
            cache: LruCache::new(MAX_RESOURCE_COUNT),
            client: Client::new(),
            storage_cache: StorageCache::with_cache_index("./cache/cache-index.json"),
        }
    }

    // This assumes that the resource at the URL never changes
    //   We may want to have an ETAG comparison
    pub async fn get(&mut self, url: &str) -> Result<Rc<Bytes>, BadPdfLayout> {
        // We have to do this since NLL are not yet implemented in Rust yet
        if self.cache.contains(url) {
            let resource = self.cache.get(url);
            Ok(resource.unwrap().clone())
        } else {
            let mut request_builder = self.client.get(url);

            let cached_file = self.storage_cache.files.get(url);

            if let Some(cached_file) = cached_file {
                request_builder = request_builder.header("If-None-Match", &cached_file.etag);
            }

            let response = request_builder.send().await?;

            if response.status() == StatusCode::NOT_MODIFIED {
                let mut cached_path = PathBuf::from("./cache");
                cached_path.push(cached_file.unwrap().cache_location.clone());

                let resource = Rc::new(Bytes::from(std::fs::read(cached_path).unwrap()));

                self.cache.put(String::from(url), resource.clone());

                Ok(resource)
            } else {
                let headers = response.headers().clone();
                let resource = Rc::new(response.bytes().await?);

                if let Some(etag) = headers.get("Etag") {
                    let info = FileInfo {
                        timestamp: Utc::now(),
                        etag: String::from(etag.to_str().unwrap()),
                        cache_location: format!("{}.bin", Utc::now().timestamp_nanos()),
                    };

                    let mut path = PathBuf::from("./cache");
                    path.push(info.cache_location.clone());

                    std::fs::write(path, resource.as_ref()).unwrap();

                    self.storage_cache.files.insert(url.to_owned(), info);

                    let cache_json = serde_json::to_string(&self.storage_cache).unwrap();

                    std::fs::write("./cache/cache-index.json", cache_json).unwrap();
                }

                self.cache.put(String::from(url), resource.clone());

                Ok(resource)
            }
        }
    }
}
