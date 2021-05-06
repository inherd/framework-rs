use crate::Framework;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Frameworks {
    pub frameworks: Vec<Framework>,
    pub facet: Vec<Value>
}
