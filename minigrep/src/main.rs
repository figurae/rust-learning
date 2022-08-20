// #![allow(dead_code, unused)]

use minigrep::Args;
use std::{env, process};

fn main() {
    let args: Args = Args::new(&mut env::args().skip(1)).unwrap_or_else(|err| {
        eprintln!("error while grabbing args: {}", err);

        process::exit(1);
    });

    if let Err(err) = minigrep::run(args) {
        eprintln!("application says: {}", err);

        process::exit(1);
    };
}
