use serde_json::Value;
use std::collections::HashSet;
use crate::DetectFramework;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Framework {
    pub name: String,
    pub path: String,
    pub relative: String,
    pub files: HashSet<String>,
    pub languages: HashSet<String>,
}

impl Framework {
    pub fn from(fw: &DetectFramework) -> Framework {
        let framework = fw.clone();
        let files_set = framework.files.into_inner();
        let languages_set = framework.languages.into_inner();

        Framework {
            name: fw.name.clone(),
            path: fw.path.clone(),
            relative: "".to_string(),
            files: files_set,
            languages: languages_set
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Frameworks {
    pub frameworks: Vec<Framework>,
    pub facets: Vec<Value>
}
