use std::env;
use std::process;

use rust_CLI::CLI;

fn main() {
    let cli_args: Vec<String> = env::args().collect();

    let cli: CLI = CLI::new(&cli_args).unwrap_or_else(|err: &str| {
        println!("\nProblem parsing arg : {}", err);
        println!("\nUse format as : cargo run searchingText filename.extension \n");
        process::exit(1);
    });

    println!("\nSearching the text : {}", cli.text);
    println!("Searching text in filename : {}\n", cli.file);

    if let Err(e) = rust_CLI::run(cli) {
        println!("\nApplication error: {}\n", e);
        process::exit(1);
    }
}
