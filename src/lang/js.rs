use crate::framework_detector::FrameworkContainer;
use walkdir::DirEntry;

pub fn tagging<'a>(entry: &DirEntry) -> Option<&'a str> {
    let file_name = entry.file_name().to_str().unwrap();
    match file_name {
        "bower.json" | "bower_components" => Some("workspace.bower"),
        "package.json" | "node_modules" => Some("workspace.npm"),
        _ => None,
    }
}

pub fn framework_analysis(_entry: &DirEntry, _frameworks: &FrameworkContainer) {}
