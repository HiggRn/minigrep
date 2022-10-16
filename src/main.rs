use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("parsing error: {err}");
            process::exit(1);
    });

    println!("Query: {}", config.query);

    if let Err(err) = minigrep::run(config) {
        eprintln!("application error : {err}");
        process::exit(1);
    }
}
