use params::Params;

mod params;

pub fn run(args: Vec<String>) {
    let params = Params::verify(&args).expect("Failed to verify parameters");
    println!("{} in {}", params.word, params.file)
}
