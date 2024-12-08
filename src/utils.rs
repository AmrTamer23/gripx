use std::path::Path;

pub fn check_if_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    let is_existing = path.exists();

    is_existing
}
