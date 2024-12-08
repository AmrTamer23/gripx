use std::env;

use gripx::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    run(args);
}
