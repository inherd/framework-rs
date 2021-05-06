use crate::Framework;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Frameworks {
    pub frameworks: Vec<Framework>,
}
