use crate::framework_detector::FrameworkContainer;
use walkdir::DirEntry;

pub fn tagging<'a>(entry: &DirEntry) -> Option<&'a str> {
    let file_name = entry.file_name().to_str().unwrap();
    match file_name {
        "go.mod" | "main.go" => Some("workspace.go"),
        _ => None,
    }
}

pub fn framework_analysis(_entry: &DirEntry, _frameworks: &FrameworkContainer) {}
