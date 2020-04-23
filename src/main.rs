use std::env;
use std::process;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Must provide the name of a file or folder to watermark and a watermark overlay file.");
        process::exit(1);
    }
    let path = &args[1];
    let watermark = &args[2];
    if let Err(e) = lib::run(path, watermark) {
        eprintln!("Application error: {:?}", e);
        process::exit(1);
    }
}
