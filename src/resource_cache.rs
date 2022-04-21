use std::rc::Rc;

use bytes::Bytes;
use lru::LruCache;
use reqwest::Client;

use crate::error::BadPdfLayout;

const MAX_RESOURCE_COUNT: usize = 100;

pub struct ResourceCache {
    cache: LruCache<String, Rc<Bytes>>,
    client: Client,
}

impl ResourceCache {
    pub fn new() -> Self {
        Self {
            cache: LruCache::new(MAX_RESOURCE_COUNT),
            client: Client::new(),
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
            let resource = self.client.get(url).send().await?.bytes().await?;

            self.cache.put(String::from(url), Rc::new(resource));
            Ok(self.cache.get(url).unwrap().clone())
        }
    }
}
