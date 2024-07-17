use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;

use cdumay_error::{Error, ErrorBuilder, ErrorKind, GenericErrors};
use cdumay_http_client::{BaseClient, ClientBuilder, ClientError};
use cdumay_http_client::authentication::Authentication;
use reqwest::{Method, Url};
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use cdumay_core::Value;

#[derive(Debug)]
pub struct RestClient {
    url_root: Url,
    timeout: u64,
    headers: HeaderMap,
    auth: Option<Box<dyn Authentication>>,
    try_number: u64,
    retry_delay: u64,
}


impl ClientBuilder for RestClient {
    fn new(url_root: &str) -> Result<RestClient, ClientError> {
        Ok(RestClient {
            url_root: Url::parse(url_root.trim_end_matches("/")).map_err(|err| {
                ClientError::UrlError(err)
            })?,
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
            try_number: 10,
            retry_delay: 30,
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
    fn set_try_number(mut self, try_number: u64) -> RestClient {
        if try_number == 0 {
            panic!("Try number MUST be > 0 !");
        }
        self.try_number = try_number;
        self
    }
    fn set_retry_delay(mut self, retry_delay: u64) -> RestClient {
        self.retry_delay = retry_delay;
        self
    }
}

impl BaseClient for RestClient {
    fn url_root(&self) -> &Url { &self.url_root }
    fn timeout(&self) -> &u64 { &self.timeout }
    fn headers(&self) -> &HeaderMap { &self.headers }
    fn auth(&self) -> Option<&Box<dyn Authentication>> { self.auth.as_ref() }
    fn try_number(&self) -> u64 { self.try_number }
    fn retry_delay(&self) -> u64 { self.retry_delay }
}

impl RestClient {
    fn create_context(&self, path: String, method: Method) -> BTreeMap<String, Value> {
        let mut context = BTreeMap::new();
        context.insert("server".into(), Value::from(self.url_root.to_string()));
        context.insert("path".into(), Value::from(path));
        context.insert("method".into(), Value::from(method.to_string()));
        context
    }

    pub fn get<R>(&self, path: String, params: Option<HashMap<String, String>>, headers: Option<HeaderMap>, timeout: Option<u64>, no_retry_on: Option<Vec<ErrorKind>>) -> Result<R, Error>
    where
            for<'a> R: Deserialize<'a>,
    {
        Ok(
            serde_json::from_str(&self.do_request(Method::GET, path.to_string(), params, None, headers, timeout, no_retry_on)?)
                .map_err(|err|
                ErrorBuilder::from(GenericErrors::DESERIALIZATION_ERROR)
                    .message(format!("Failed to deserialize response: {}", err.to_string()))
                    .extra(self.create_context(path, Method::GET).into())
                    .build()
                )?
        )
    }
    pub fn post<D, R>(&self, path: String, params: Option<HashMap<String, String>>, data: Option<D>, headers: Option<HeaderMap>, timeout: Option<u64>, no_retry_on: Option<Vec<ErrorKind>>) -> Result<R, Error>
    where
            for<'a> R: Deserialize<'a>,
            D: Serialize + Debug,
    {
        let payload = match data {
            Some(txt) => Some(
                serde_json::to_string(&txt).map_err(|err|
                ErrorBuilder::from(GenericErrors::DESERIALIZATION_ERROR)
                    .message(format!("Failed to serialize payload: {} -> {:?}", err.to_string(), txt))
                    .extra(self.create_context(path.to_string(), Method::POST).into())
                    .build()
                )?
            ),
            None => None
        };
        Ok(
            serde_json::from_str(&self.do_request(Method::POST, path.to_string(), params, payload, headers, timeout, no_retry_on)?)
                .map_err(|err|
                ErrorBuilder::from(GenericErrors::SERIALIZATION_ERROR)
                    .message(format!("Failed to deserialize response: {}", err.to_string()))
                    .extra(self.create_context(path, Method::POST).into())
                    .build()
                )?
        )
    }
    pub fn put<D, R>(&self, path: String, params: Option<HashMap<String, String>>, data: Option<D>, headers: Option<HeaderMap>, timeout: Option<u64>, no_retry_on: Option<Vec<ErrorKind>>) -> Result<R, Error>
    where
            for<'a> R: Deserialize<'a>,
            D: Serialize + Debug,
    {
        let payload = match data {
            Some(txt) => Some(
                serde_json::to_string(&txt).map_err(|err|
                ErrorBuilder::from(GenericErrors::DESERIALIZATION_ERROR)
                    .message(format!("Failed to serialize payload: {} -> {:?}", err.to_string(), txt))
                    .extra(self.create_context(path.to_string(), Method::PUT).into())
                    .build()
                )?
            ),
            None => None
        };
        Ok(
            serde_json::from_str(&self.do_request(Method::PUT, path.to_string(), params, payload, headers, timeout, no_retry_on)?)
                .map_err(|err|
                ErrorBuilder::from(GenericErrors::SERIALIZATION_ERROR)
                    .message(format!("Failed to deserialize response: {}", err.to_string()))
                    .extra(self.create_context(path, Method::PUT).into())
                    .build()
                )?
        )
    }
    pub fn delete<R>(&self, path: String, params: Option<HashMap<String, String>>, headers: Option<HeaderMap>, timeout: Option<u64>, no_retry_on: Option<Vec<ErrorKind>>) -> Result<R, Error>
    where
            for<'a> R: Deserialize<'a>,
    {
        Ok(
            serde_json::from_str(&self.do_request(Method::DELETE, path.to_string(), params, None, headers, timeout, no_retry_on)?)
                .map_err(|err|
                ErrorBuilder::from(GenericErrors::SERIALIZATION_ERROR)
                    .message(format!("Failed to deserialize response: {}", err.to_string()))
                    .extra(self.create_context(path, Method::PUT).into())
                    .build()
                )?
        )
    }
}


#[cfg(test)]
mod test {
    use std::sync::Once;
    use cdumay_error::GenericErrors;
    use cdumay_http_client::{ClientBuilder, HttpStatusCodeErrors};
    use serde::{Deserialize, Serialize};
    use simple_logger::SimpleLogger;

    use crate::RestClient;

    static INIT: Once = Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            let _ = SimpleLogger::new().with_level(log::LevelFilter::Info).init();
        });
    }
    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct Todo {
        id: usize,
        todo: String,
        completed: bool,
        #[serde(rename(deserialize = "userId"))]
        user_id: u64,
    }
    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct Foo {
        id: usize,
        foo: String,
    }

    #[test]
    fn test_get() {
        init_logger();
        let cli = RestClient::new("https://dummyjson.com").unwrap();
        let result = cli.get::<Todo>("/todos/1".into(), None, None, None, None);
        match result {
            Ok(todo) => assert_eq!(todo.user_id, 152),
            Err(err) => panic!("{}", err)
        }
    }

    #[test]
    fn test_get_payload_error() {
        init_logger();
        let cli = RestClient::new("https://dummyjson.com")
            .unwrap()
            .set_try_number(2)
            .set_retry_delay(1);
        let result = cli.get::<Foo>("/todos/1".into(), None, None, None, None);
        match result {
            Ok(_) => panic!("No error raised!"),
            Err(err) => assert_eq!(err.kind, GenericErrors::DESERIALIZATION_ERROR)
        }
    }
    #[test]
    fn test_get_response_error() {
        init_logger();
        let cli = RestClient::new("https://dummyjson.com")
            .unwrap()
            .set_try_number(2)
            .set_retry_delay(1);
        let result = cli.get::<Todo>("/todos/a".into(), None, None, None, None);
        match result {
            Ok(_) => panic!("No error raised!"),
            Err(err) => assert_eq!(err.kind, HttpStatusCodeErrors::NOT_FOUND)
        }
    }
}