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
        // serde_json::from_reader(f)?
        match serde_json::from_reader(f) {
            Ok(v) => Ok(v),
            Err(e) => Err(anyhow!("Failed to parse from json, error: {}", e)),
        }
    }
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let result = Sample::from_file("sample.json")?;
    println!("{:?}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn succeed_in_read_file() {
        let result = Sample::from_file("sample.json");
        assert!(result.is_ok());
    }

    #[test]
    fn fail_to_read_file_from_missing_target_keys() {
        let result = Sample::from_file("testdata/missing_target_keys.json");
        assert!(result.is_err());
    }

    #[test]
    fn fail_to_read_file_from_invalid_keys() {
        let result = Sample::from_file("testdata/invalid_value_type.json");
        assert!(result.is_err());
    }

    #[test]
    fn fail_to_read_file_from_non_existed_path() {
        let result = Sample::from_file("not_existed_path");
        assert!(result.is_err());
    }
}