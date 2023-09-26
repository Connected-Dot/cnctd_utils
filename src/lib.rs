use std::path::Path;

pub fn get_relative_path(parent: &Path, child: &Path) -> Option<String> {
    child.strip_prefix(parent).ok().map(|path| path.to_str().unwrap().to_string())
}