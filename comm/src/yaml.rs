use crate::errors::entrances::{GeorgeResult, err_str_enhance};
use serde::de::DeserializeOwned;

pub fn string_2_yaml<T: DeserializeOwned>(data: String) -> GeorgeResult<T> {
    let t: T;
    match serde_yaml::from_str(&data) {
        Ok(serde_t) => {
            t = serde_t;
            Ok(t)
        }
        Err(err) => Err(err_str_enhance("serde_yaml_from_str", err.to_string())),
    }
}
