extern crate bs;

use bs::parse::diagnostic::Diagnostic;
use bs::result::BSResult;
use bs::rt::runtime::Runtime;
use std::io::{self, Write};

/// Entry point of the program; acts as a REPL.
pub fn main() {
    let mut runtime = Runtime::new().expect("Failed to create runtime");
    loop {
        print!("\n> ");
        let _ = io::stdout().flush();

        // Read input from stdin
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read from standard input.");

        let res = match runtime.parse_eval(input.as_str()) {
            BSResult::Ok(result) => format!("=> {}", result),
            BSResult::Err(err) => format!("{}", Diagnostic::new("REPL", &input, err)),
        };

        println!("{}", res);
    }
}
