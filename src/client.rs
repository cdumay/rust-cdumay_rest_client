use std::collections::HashMap;

use cdumay_http_client::{BaseClient, ClientBuilder, ClientError};
use cdumay_http_client::authentication::Authentication;
use reqwest::{Method, Url};
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};

use crate::errors::RestClientError;

#[derive(Debug)]
pub struct RestClient {
    url_root: Url,
    timeout: u64,
    headers: HeaderMap,
    auth: Option<Box<dyn Authentication>>,
}


impl ClientBuilder for RestClient {
    fn new(url_root: &str) -> Result<RestClient, ClientError> {
        Ok(RestClient {
            url_root: Url::parse(url_root.trim_end_matches("/"))?,
            timeout: 10,
            headers: {
                let mut headers = HeaderMap::new();
                headers.append(
                    USER_AGENT,
                    HeaderValue::from_str(&format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))).unwrap(),
                );
                headers.append(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                headers.append(ACCEPT, HeaderValue::from_static("application/json"));
                headers
            },
            auth: None,
        })
    }
    fn set_timeout(mut self, timeout: u64) -> RestClient {
        self.timeout = timeout;
        self
    }
    fn set_headers(mut self, headers: HeaderMap) -> RestClient {
        self.headers.extend(headers);
        self
    }
    fn set_auth<A: Authentication + 'static>(mut self, auth: A) -> RestClient {
        self.auth = Some(Box::new(auth));
        self
    }
}

impl BaseClient for RestClient {
    fn url_root(&self) -> &Url { &self.url_root }
    fn timeout(&self) -> &u64 { &self.timeout }
    fn headers(&self) -> &HeaderMap { &self.headers }
    fn auth(&self) -> Option<&Box<dyn Authentication>> { self.auth.as_ref() }
}

impl RestClient {
    pub fn get<R>(&self, path: String, params: Option<HashMap<String, String>>, headers: Option<HeaderMap>, timeout: Option<u64>) -> Result<R, RestClientError>
        where for<'a> R: Deserialize<'a>
    {
        Ok(serde_json::from_str(&self.do_request(Method::GET, path, params, None, headers, timeout)?)?)
    }
    pub fn post<D, R>(&self, path: String, params: Option<HashMap<String, String>>, data: Option<D>, headers: Option<HeaderMap>, timeout: Option<u64>) -> Result<R, RestClientError>
        where for<'a> R: Deserialize<'a>, D: Serialize
    {
        let jdata = match data {
            Some(txt) => Some(serde_json::to_string(&txt)?),
            None => None
        };
        Ok(serde_json::from_str(&self.do_request(Method::POST, path, params, jdata, headers, timeout)?)?)
    }
    pub fn put<D, R>(&self, path: String, params: Option<HashMap<String, String>>, data: Option<D>, headers: Option<HeaderMap>, timeout: Option<u64>) -> Result<R, RestClientError>
        where for<'a> R: Deserialize<'a>, D: Serialize
    {
        let jdata = match data {
            Some(txt) => Some(serde_json::to_string(&txt)?),
            None => None
        };
        Ok(serde_json::from_str(&self.do_request(Method::PUT, path, params, jdata, headers, timeout)?)?)
    }
    pub fn delete<R>(&self, path: String, params: Option<HashMap<String, String>>, headers: Option<HeaderMap>, timeout: Option<u64>) -> Result<R, RestClientError>
        where for<'a> R: Deserialize<'a>
    {
        Ok(serde_json::from_str(&self.do_request(Method::DELETE, path, params, None, headers, timeout)?)?)
    }
}
