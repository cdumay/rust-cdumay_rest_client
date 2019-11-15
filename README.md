# cdumay_rest_client

[![Build Status](https://travis-ci.org/cdumay/rust-cdumay_rest_client.svg?branch=master)](https://travis-ci.org/cdumay/rust-cdumay_rest_client)
[![Latest version](https://img.shields.io/crates/v/cdumay_rest_client.svg)](https://crates.io/crates/cdumay_rest_client)
[![Documentation](https://docs.rs/cdumay_rest_client/badge.svg)](https://docs.rs/cdumay_rest_client)
![License](https://img.shields.io/crates/l/cdumay_rest_client.svg)

cdumay_rest_client is a basic REST library used to standardize result and serialize them using [serde](https://docs.serde.rs/serde/).

## Quickstart

_Cargo.toml_:
```toml
[dependencies]
serde = "1.0"
serde_derive = "1.0"
serde_json = "1.0"
cdumay_error = "0.1"
cdumay_http_client = "0.1"
cdumay_rest_client = "0.1"
```

_main.rs_:

```rust
extern crate cdumay_error;
extern crate cdumay_http_client;
extern crate cdumay_rest_client;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use cdumay_error::ErrorRepr;
use cdumay_http_client::{ClientBuilder, HttpClient};
use cdumay_http_client::authentication::NoAuth;
use cdumay_rest_client::RestClient;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: usize,
    task: String,
}

fn main() {
    let cli = RestClient::<NoAuth>::new("http://127.0.0.1:5000").unwrap();
    let result = cli.get::<Todo>("/todos/1".into(), None, None, None);

    match result {
        Ok(todo) => println!("{}", serde_json::to_string_pretty(&todo).unwrap()),
        Err(err) => println!("{}", serde_json::to_string_pretty(&ErrorRepr::from(err)).unwrap()),
    }
}
```
_Output_:
```json
{
  "id": 1,
  "task": "Build an API"
}
```
## Errors

Errors can be displayed using [cdumay_error](https://docs.serde.rs/cdumay_error/):

```json
{
  "code": 404,
  "extra": {
    "message": "Todo 7000 doesn't exist. You have requested this URI [/todos/7000] but did you mean /todos/<int:id> ?"
  },
  "message": "Not Found",
  "msgid": "Err-18430"
}
```

## Project Links

- Issues: https://github.com/cdumay/rust-cdumay_rest_client/issues
- Documentation: https://docs.rs/cdumay_rest_client

License: MIT
