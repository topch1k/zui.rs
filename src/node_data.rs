use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NodeData {
    Raw(Vec<u8>),
    String(String),
    Json(Value),
}

impl Default for NodeData {
    fn default() -> Self {
        Self::Raw(Vec::new())
    }
}

impl NodeData {
    pub fn parse_data_as_json(data: &[u8]) -> Option<Value> {
        serde_json::to_value(data).ok()
    }

    pub fn parse_data_as_string(data: &[u8]) -> Option<String> {
        Some(String::from_utf8_lossy(data).into_owned())
    }

    pub fn convert_to_json(self) -> Self {
        match &self {
            NodeData::Raw(vec) => serde_json::from_slice(vec)
                .map(NodeData::Json)
                .unwrap_or(self),
            NodeData::String(str) => serde_json::from_str(str)
                .map(NodeData::Json)
                .unwrap_or(self),
            NodeData::Json(_) => self,
        }
    }
    pub fn convert_to_string(self) -> Self {
        match self {
            NodeData::Raw(vec) => NodeData::String(String::from_utf8_lossy(&vec).to_string()),
            NodeData::String(_) => self,
            NodeData::Json(value) => NodeData::String(value.to_string()),
        }
    }
    pub fn convert_to_raw(self) -> Self {
        match self {
            NodeData::Raw(_) => self,
            NodeData::String(str) => NodeData::Raw(str.into_bytes()),
            NodeData::Json(value) => {
                NodeData::Raw(serde_json::to_vec(&value).unwrap_or(Vec::new()))
            }
        }
    }
}

impl fmt::Display for NodeData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeData::Raw(vec) => write!(f, "{:?}", vec),
            NodeData::String(str) => write!(f, "{}", str),
            NodeData::Json(value) => write!(f, "{}", value),
        }
    }
}
