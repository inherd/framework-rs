use std::path::PathBuf;
use crate::dependency::base_library::Library;

pub trait LibraryFinderTrait {
    fn new() -> Self;
    fn search(&self, path: &PathBuf) -> Vec<Library>;
}
