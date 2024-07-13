# cdumay_rest_client ![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue) [![cdumay_rest_client on crates.io](https://img.shields.io/crates/v/cdumay_rest_client)](https://crates.io/crates/cdumay_rest_client) [![cdumay_rest_client on docs.rs](https://docs.rs/cdumay_rest_client/badge.svg)](https://docs.rs/cdumay_rest_client) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/rust-cdumay_rest_client)

cdumay_rest_client is a basic REST library used to standardize result and serialize them using [serde][__link0].

### Quickstart

*Cargo.toml*:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
cdumay_error = "0.3"
cdumay_http_client = "0.3"
cdumay_rest_client = "0.3"
```

*main.rs*:

```rust
extern crate cdumay_error;
extern crate cdumay_http_client;
extern crate cdumay_rest_client;
extern crate serde_json;

use cdumay_error::JsonError;
use cdumay_http_client::{ClientBuilder, HttpClient};
use cdumay_http_client::authentication::NoAuth;
use cdumay_rest_client::RestClient;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: usize,
    task: String,
}

fn main() {
    let cli = RestClient::new("http://127.0.0.1:5000").unwrap();
    let result = cli.get::<Todo>("/todos/1".into(), None, None, None, None);

    match result {
        Ok(todo) => println!("{}", serde_json::to_string_pretty(&todo).unwrap()),
        Err(err) => println!("{}", serde_json::to_string_pretty(&JsonError::from(err)).unwrap()),
    }
}
```

*Output*:

```json
{
  "id": 1,
  "task": "Build an API"
}
```

### Errors

Errors can be displayed using [cdumay_error][__link1]:

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

### Project Links

* Issues: https://github.com/cdumay/rust-cdumay_rest_client/issues
* Documentation: https://docs.rs/cdumay_rest_client


 [__link0]: https://docs.serde.rs/serde/
 [__link1]: https://docs.serde.rs/cdumay_error/
