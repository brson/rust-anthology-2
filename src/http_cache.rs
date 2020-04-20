use log::debug;
use std::io::ErrorKind;
use std::fs::{self, File};
use blake2::{Blake2b, Digest};
use anyhow::{Result, Context, anyhow};
use reqwest::StatusCode;
use reqwest::blocking::Client as HttpClient;
use url::Url;
use std::path::PathBuf;

pub struct HttpCache {
    dir: PathBuf,
    client: HttpClient,
}

impl HttpCache {
    pub fn new(dir: PathBuf) -> HttpCache {
        HttpCache {
            dir, client: HttpClient::new(),
        }
    }

    pub fn get(&mut self, url: &Url) -> Result<String> {
        let hash = url_hash(url);
        debug!("fetching {}", url);
        debug!("hash {}", hash);
        let cache_path = self.dir.join(&hash);
        let cached = fs::read_to_string(&cache_path);
        match cached {
            Ok(cached) => {
                debug!("cache hit for {}", url);
                Ok(cached)
            },
            Err(e) if e.kind() == ErrorKind::NotFound => {
                let body = fetch_url(&mut self.client, url)?;
                debug!("writing cache for {} to {}", url, cache_path.display());
                fs::create_dir_all(&self.dir)
                    .context("creating cache dir")?;
                fs::write(cache_path, &body)
                    .context("writing cache")?;
                Ok(body)
            },
            e => e.context("opening cache"),
        }
    }
}

pub fn url_hash(url: &Url) -> String {
    let mut hasher = Blake2b::new();
    hasher.input(url.as_str());
    let res = hasher.result();
    hex::encode(&res[..20])
}

fn fetch_url(client: &mut HttpClient, url: &Url) -> Result<String> {
    let resp = client.get(url.clone()).send()?;
    debug!("printing headers");
    for (key, value) in resp.headers() {
        debug!("{}: {:?}", key, value);
    }
    if resp.status().is_success() {
        Ok(resp.text()
            .context("parsing response as text")?)
    } else {
        Err(anyhow!("failed to fetch url {}", url))
    }
}
