use crate::facet::{Facet, FacetsBuilder};
use crate::frameworks::{Framework, Frameworks};
use crate::lang::LangDetectors;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct DetectFramework {
    pub name: String,
    pub path: String,
    pub relative: String,
    // in some languages has different framework file
    // |   languages |   files    |
    // |-------------|------------|
    // | Java        | build.gradle, settings.gradle |
    pub files: RefCell<HashSet<String>>,
    // in JVM projects, has different languages, such as Java, Groovy, Kotlin...
    pub languages: RefCell<HashSet<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SourceFile {
    file_path: String,
    language: String,
}

#[derive(Serialize)]
pub struct FrameworkContainer {
    entries: RefCell<Vec<DetectFramework>>,

    #[serde(skip_serializing)]
    temp_source_files: RefCell<Vec<SourceFile>>,
}

impl FrameworkContainer {
    /// add framework to project
    pub fn add_framework(&self, framework: DetectFramework) {
        if !self.entries.borrow().contains(&framework) {
            self.associate_with_source_files(&framework);
            self.entries.borrow_mut().push(framework);
        }
    }

    fn associate_with_source_files(&self, framework: &DetectFramework) {
        for temp_source_file in self.temp_source_files.borrow().iter() {
            if temp_source_file.file_path.starts_with(&framework.path) {
                FrameworkContainer::add_language_to_framework(
                    temp_source_file.language.clone(),
                    framework,
                )
            }
        }
    }

    pub fn add_language(&self, file_path: &str, language: &str) {
        self.add_language_to_frameworks(file_path, &language);
        self.cache_source_file(file_path, language);
    }

    fn add_language_to_frameworks(&self, file_path: &str, language: &str) {
        for framework in self.entries.borrow_mut().iter() {
            if file_path.starts_with(&framework.path) {
                FrameworkContainer::add_language_to_framework(language.to_string(), framework);
            }
        }
    }

    fn add_language_to_framework(language: String, framework: &DetectFramework) {
        framework.languages.borrow_mut().insert(language);
    }

    fn cache_source_file(&self, file_path: &str, language: &str) {
        self.temp_source_files.borrow_mut().push(SourceFile {
            file_path: file_path.to_string(),
            language: language.to_string(),
        });
    }

    pub fn append(&self, frameworks: &FrameworkContainer) {
        self.entries
            .borrow_mut()
            .append(&mut frameworks.entries.borrow_mut())
    }

    pub fn add_settings_file(&self, framework_name: &str, file_path: &str, file_name: &str) {
        for framework in self.entries.borrow_mut().iter() {
            if file_path.starts_with(&framework.path) && framework.name.eq(framework_name) {
                framework.files.borrow_mut().insert(file_name.to_string());
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<DetectFramework> {
        let frameworks = self.entries.borrow();
        match frameworks.get(index) {
            Some(framework) => Some(framework.clone()),
            _ => None,
        }
    }
}

impl Default for FrameworkContainer {
    fn default() -> Self {
        FrameworkContainer {
            entries: RefCell::new(vec![]),
            temp_source_files: RefCell::new(vec![]),
        }
    }
}

/// Framework Detector
#[derive(Serialize)]
pub struct FrameworkDetector<'a> {
    pub(crate) tags: BTreeMap<&'a str, bool>,

    pub container: FrameworkContainer,
    pub facets: Vec<Box<Facet>>,
}

impl<'a> Default for FrameworkDetector<'a> {
    fn default() -> Self {
        FrameworkDetector {
            tags: BTreeMap::default(),
            container: FrameworkContainer::default(),
            facets: vec![],
        }
    }
}

impl<'a> FrameworkDetector<'a> {
    /// return frameworks info os api
    ///
    /// Examples
    ///
    /// ```
    /// use framework::FrameworkDetector;
    ///
    /// FrameworkDetector::detect("path");
    /// ```
    pub fn detect<P: AsRef<Path>>(path: P) -> FrameworkDetector<'a> {
        let mut detector = FrameworkDetector::default();
        detector.run(path);
        detector
    }

    pub fn build(&self) -> Frameworks {
        let frameworks = self
            .container
            .entries
            .borrow()
            .to_vec()
            .iter()
            .map(|fw| Framework::from(fw))
            .collect();
        let facets = self
            .facets
            .iter()
            .map(|s| serde_json::to_value(s).unwrap())
            .collect();

        Frameworks { frameworks, facets }
    }

    fn run<P: AsRef<Path>>(&mut self, path: P) {
        let mut lang_detectors = FrameworkDetector::detect_languages(&path);

        self.add_language_tags(&mut lang_detectors);
        self.add_frameworks(&mut lang_detectors);
        self.add_facets();
    }

    fn detect_languages<P: AsRef<Path>>(path: P) -> LangDetectors<'a> {
        let mut lang_detectors = LangDetectors::default();
        lang_detectors.detect(&path);
        lang_detectors
    }

    fn add_language_tags(&mut self, detectors: &mut LangDetectors<'a>) {
        self.tags.append(&mut detectors.tags);
    }

    fn add_facets(&mut self) {
        let builder = FacetsBuilder::default();
        let mut facets = builder.build(&self.tags);
        self.facets.append(&mut facets);
    }

    fn add_frameworks(&mut self, detectors: &mut LangDetectors<'a>) {
        self.container.append(&detectors.frameworks);
    }
}

#[cfg(test)]
mod tests {
    use crate::framework_detector::FrameworkDetector;
    use crate::lang::jvm;
    use std::path::PathBuf;

    fn build_test_detector<'a>(project_path: Vec<&str>) -> FrameworkDetector<'a> {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut test_project_dir = root_dir.clone();

        for path in project_path.into_iter() {
            test_project_dir.push(path);
        }

        let detector = FrameworkDetector::detect(test_project_dir);
        detector
    }

    #[test]
    fn should_detect_java_gradle_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "java", "simple"]);

        assert!(detector.tags.get(jvm::WORKSPACE_FRAMEWORK_GRADLE).unwrap());
        assert!(detector
            .tags
            .get(jvm::WORKSPACE_FRAMEWORK_GRADLE_COMPOSITE)
            .unwrap());
    }

    #[test]
    fn should_build_framework_info() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "java", "hello"]);

        assert_eq!(1, detector.facets.len());
    }

    #[test]
    fn should_detect_go_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "go", "hello"]);
        assert_eq!(&true, detector.tags.get("workspace.go").unwrap());

        let detector = build_test_detector(vec!["_fixtures", "projects", "go", "simple"]);
        assert_eq!(&true, detector.tags.get("workspace.go").unwrap());
    }

    #[test]
    fn should_detect_rust_cargo_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "rust", "cargo"]);

        assert_eq!(&true, detector.tags.get("workspace.cargo").unwrap());
    }

    #[test]
    fn should_detect_bower_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "js", "bowerproject"]);

        assert_eq!(&true, detector.tags.get("workspace.bower").unwrap());
    }

    #[test]
    fn should_detect_npm_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "js", "npmproject"]);

        let frameworks = detector.build();
        println!("{:?}", frameworks);
        assert_eq!(&true, detector.tags.get("workspace.npm").unwrap());
    }

    #[test]
    fn should_detect_jvm_project() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "jvm"]);

        let facets_json = serde_json::to_string_pretty(&detector.facets).unwrap();
        let expect_json = r#"[
  {
    "jvm": {
      "is_gradle": true,
      "is_maven": true,
      "has_java": true,
      "has_groovy": true,
      "has_kotlin": true,
      "has_scala": true
    },
    "include_test": true
  }
]"#;
        assert_eq!(expect_json, facets_json)
    }

    #[ignore]
    #[test]
    fn should_detect_jvm_frameworks() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "jvm"]);
        let frameworks = detector.container;

        let framework = frameworks.get(0).unwrap();
        let name = framework.name.to_string();
        let files = framework.files.borrow();
        let languages = framework.languages.borrow();
        assert_eq!(name, "Gradle");
        assert_eq!(files.contains("build.gradle"), true);
        assert_eq!(files.contains("settings.gradle"), true);
        assert_eq!(languages.contains("Groovy"), true);
        assert_eq!(languages.contains("Java"), true);
        assert_eq!(languages.contains("Kotlin"), true);
        assert_eq!(languages.contains("Scala"), true);

        let framework = frameworks.get(1).unwrap();
        let name = framework.name.to_string();
        let files = framework.files.borrow();
        let languages = framework.languages.borrow();
        assert_eq!(name, "Maven");
        assert_eq!(files.contains("pom.xml"), true);
        assert_eq!(languages.contains("Java"), true);
        assert_eq!(languages.contains("Kotlin"), true);
    }

    #[test]
    fn should_build_frameworks() {
        let detector = build_test_detector(vec!["_fixtures", "projects", "jvm"]);
        let frameworks = detector.build();

        assert_eq!(2, frameworks.frameworks.len());
    }
}
