use file_utils::{check_if_exists, search_in_file};
use params::Params;

mod file_utils;
mod params;

pub fn run(args: Vec<String>) {
    let params = Params::verify(&args).expect("Failed to verify parameters");
    println!("{} in {}", params.word, params.file);

    if check_if_exists(&params.file) {
        let matches = search_in_file(&params.word, &params.file);
        if matches.len() < 1 {
            panic!("There's no match in the given file");
        } else {
            for line in matches {
                println!("{}", line);
            }
        }
    } else {
        panic!("The file path is incorrect");
    }
}
