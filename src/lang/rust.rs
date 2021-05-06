use crate::framework_detector::FrameworkContainer;
use walkdir::DirEntry;

pub fn tagging<'a>(entry: &DirEntry) -> Option<&'a str> {
    let file_name = entry.file_name().to_str().unwrap();
    match file_name {
        "Cargo.toml" => Some("workspace.cargo"),
        _ => None,
    }
}

pub fn framework_analysis(_entry: &DirEntry, _frameworks: &FrameworkContainer) {}
