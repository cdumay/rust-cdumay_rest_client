use std::collections::btree_map::BTreeMap;
use std::error::Error;
use std::fmt;

use cdumay_error::{ErrorInfo, GenericErrors};
use cdumay_http_client::ClientError;
use serde_value::Value;

#[derive(Debug)]
pub enum RestClientError {
    ClientError(ClientError),
    SerializationError(serde_json::error::Error),
}

impl From<ClientError> for RestClientError {
    fn from(err: ClientError) -> RestClientError {
        RestClientError::ClientError(err)
    }
}

impl From<serde_json::error::Error> for RestClientError {
    fn from(err: serde_json::error::Error) -> RestClientError {
        RestClientError::SerializationError(err)
    }
}

impl ErrorInfo for RestClientError {
    fn code(&self) -> u16 {
        match self {
            RestClientError::ClientError(err) => err.code(),
            RestClientError::SerializationError(_) => GenericErrors::SERIALIZATION_ERROR.code()
        }
    }

    fn extra(&self) -> Option<BTreeMap<String, Value>> {
        match self {
            RestClientError::ClientError(err) => err.extra(),
            _ => None
        }
    }

    fn message(&self) -> String {
        match self {
            RestClientError::ClientError(err) => err.message(),
            RestClientError::SerializationError(err) => err.description().to_string(),
        }
    }

    fn msgid(&self) -> String {
        match self {
            RestClientError::ClientError(err) => err.msgid(),
            RestClientError::SerializationError(_) => GenericErrors::SERIALIZATION_ERROR.msgid()
        }
    }
}

impl fmt::Display for RestClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.msgid(), self.message())
    }
}