# Error Log

## expected struct `File`, found enum `Result`

```rust
use serde::{Serialize, Deserialize};
use std::fs::File;
use anyhow::{Result, anyhow};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sample {
    pub key: String,
}

impl Sample {
    pub fn from_file(file_path: &str) -> Result<Self> {
        let f = match File::open(file_path) {
            Ok(v) => v,
            // here error happens
            Err(e) => Err(anyhow!("Failed to open file, error: {}", e)),
        };
        
        match serde_json::from_reader(f) {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow!("Failed to parse from json, error: {}", e)),
        }
    }
}
```

```bash
cargo run
   Compiling rust-simple-test-practice v0.1.0 (/Users/kazune/CLionProjects/sandbox/rust-simple-test-practice)
error[E0308]: `match` arms have incompatible types
  --> src/main.rs:15:23
   |
13 |           let f = match File::open(file_path) {
   |  _________________-
14 | |             Ok(v) => v,
   | |                      - this is found to be of type `File`
15 | |             Err(e) => Err(anyhow!("Failed to open file, error: {}", e)),
   | |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `File`, found enum `Result`
16 | |         };
   | |_________- `match` arms have incompatible types
   |
   = note: expected struct `File`
                found enum `Result<_, anyhow::Error>`

```


## the trait `Deserialize<'_>` is not implemented for `anyhow::Error`
```rust
use serde::{Serialize, Deserialize};
use std::fs::File;
use anyhow::{Result, anyhow};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sample {
    pub key: String,
}

impl Sample {
    pub fn from_file(file_path: &str) -> Result<Self> {
        // let f = File::open(file_path)?;
        let f = match File::open(file_path) {
            Ok(v) => v,
            Err(e) => return Err(anyhow!("Failed to open file, error: {}", e)),
        };
        // here error happens
        serde_json::from_reader(f)?
    }
}
```

```bash
cargo run
   Compiling rust-simple-test-practice v0.1.0 (/Users/kazune/CLionProjects/sandbox/rust-simple-test-practice)
error[E0277]: the trait bound `anyhow::Error: Deserialize<'_>` is not satisfied
    --> src/main.rs:17:9
     |
17   |         serde_json::from_reader(f)?
     |         ^^^^^^^^^^^^^^^^^^^^^^^ the trait `Deserialize<'_>` is not implemented for `anyhow::Error`
     |
     = help: the following other types implement trait `Deserialize<'de>`:
               &'a Path
               &'a [u8]
               &'a str
               ()
               (T0, T1)
               (T0, T1, T2)
               (T0, T1, T2, T3)
               (T0, T1, T2, T3, T4)
             and 129 others
     = note: required for `Result<Sample, anyhow::Error>` to implement `for<'de> Deserialize<'de>`
     = note: required for `Result<Sample, anyhow::Error>` to implement `DeserializeOwned`
note: required by a bound in `from_reader`
    --> /Users/kazune/.cargo/registry/src/github.com-1ecc6299db9ec823/serde_json-1.0.89/src/de.rs:2519:8
     |
2519 |     T: de::DeserializeOwned,
     |        ^^^^^^^^^^^^^^^^^^^^ required by this bound in `from_reader`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rust-simple-test-practice` due to previous error
```
