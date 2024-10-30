use eyre::Result;
use serde::{Deserialize, Serialize};

pub mod param_models;

pub fn convert<From: Serialize, To: for<'a> Deserialize<'a>>(from: &From) -> Result<To> {
    let serialised = serde_json::to_string(from)?;
    let to = serde_json::from_str::<To>(&serialised)?;
    Ok(to)
}

// pub fn convert2<'a, From: Serialize, To: Deserialize<'a>>(from: &From) -> Result<To> {
//     let serialised = serde_json::to_string(from)?;
//     let to = serde_json::from_str::<To>(&serialised)?;
//     Ok(to)
// }
