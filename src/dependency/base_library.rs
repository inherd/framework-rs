#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Library {
    pub name: String,
    pub version: String,
    pub group: String,
    pub source: String,
    pub scope: String,
}

pub enum LibraryScope {
    Dev,
    Test,
}

impl Library {
    pub fn is_dev(&self) -> bool {
        return self.scope == "Test";
    }
}

#[cfg(test)]
mod tests {
    use crate::dependency::base_library::Library;

    #[test]
    fn should_be_dev_when_scope_dev() {
        let base_library = Library {
            name: "some".to_string(),
            version: "0.1.1".to_string(),
            group: "".to_string(),
            source: "github.com".to_string(),
            scope: "Test".to_string(),
        };

        assert!(base_library.is_dev());
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LibraryDependency {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ModuleDependency {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ProjectDependency {}
