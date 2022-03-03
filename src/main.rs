#![deny(clippy::all, clippy::pedantic)]
use std::process::exit;

use colored::Colorize;
mod lex;
mod parse;
mod reduce;
mod types;

const HELP: &str = r#"
Lambda syntax:
    [\ | λ] variable expression

Options:
    -s --steps      Show all steps in lambda reductions
    -d --debug      Shows lexing and parsing information
    -h --help       Show this help information
"#;

fn main() {
    let rules = lex::rules();
    let stdin = std::io::stdin();

    let argv: Vec<String> = std::env::args().collect();
    let steps = argv.contains(&"-s".to_string()) || argv.contains(&"--steps".to_string());
    let debug = argv.contains(&"-d".to_string()) || argv.contains(&"--debug".to_string());
    let help = argv.contains(&"-h".to_string()) || argv.contains(&"--help".to_string());

    if help {
        print!("{}", HELP);
    } else {
        loop {
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();

            if input == "" {
                exit(0)
            } else if input != "\n" {
                match parse::parse(&rules, &input, debug) {
                    Ok(tree) => {
                        let mut tree = tree;
                        while let Some(x) = reduce::reduce(&tree) {
                            if steps {
                                println!("{}", tree);
                            }
                            tree = x;
                        }
                        println!("{}", tree);
                    }
                    Err(e) => {
                        println!("{}", format!("{}", e).red());
                    }
                }
            }
        }
    }
}
