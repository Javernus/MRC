use serde::{Serialize, Deserialize};

pub const DEFAULT_MPW: &str = "";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Mpw {
    pub mpw: String,
}

impl Mpw {
    pub fn new(mpw: &str) -> Mpw {
        Mpw {
            mpw: mpw.to_string(),
        }
    }
}

pub fn serialize(mpw: &Mpw) -> String {
    serde_json::to_string(mpw).unwrap()
}

pub fn deserialize(text: &str) -> Mpw {
    if text.is_empty() {
        Mpw::new(DEFAULT_MPW)
    } else {
        serde_json::from_str(text).unwrap()
    }
}
