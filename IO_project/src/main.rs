use std::env;
use std::process;

use io_project::run;
use io_project::Config;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args);

    let config = match config {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Problem parsing arguments: {}", e);
            return;
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
