use crate::DetectFramework;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Frameworks {
    pub frameworks: Vec<DetectFramework>,
    pub facet: Vec<Value>
}
