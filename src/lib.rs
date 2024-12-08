use params::Params;
use utils::check_if_exists;

mod params;
mod utils;

pub fn run(args: Vec<String>) {
    let params = Params::verify(&args).expect("Failed to verify parameters");
    println!("{} in {}", params.word, params.file);

    if check_if_exists(&params.file) {
    } else {
        panic!("The file path is incorrect");
    }
}
