use rust_file_search::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ignore_case = env::var("INGORE_CASE").is_ok();
    let query = args.get(1).unwrap().to_string();
    let file = args.get(2).unwrap().to_string();
    let config = Config::build(query, file, ignore_case).unwrap();

    if let Err(e) = rust_file_search::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
