use std::{fs::File, io::Read, path::Path};

pub fn check_if_exists(file_path: &str) -> bool {
    let path = Path::new(file_path);
    let is_existing = path.exists();

    is_existing
}

pub fn search_in_file(word: &str, file: &str) -> Vec<String> {
    let mut file = File::open(file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let matches: Vec<String> = content
        .split('\n')
        .filter(|line| line.contains(word))
        .map(String::from)
        .collect();

    matches
}
