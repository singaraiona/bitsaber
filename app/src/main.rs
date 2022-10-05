extern crate bs;

use bs::rt::runtime::Runtime;
use std::io::{self, Write};

/// Entry point of the program; acts as a REPL.
pub fn main() {
    let mut runtime = Runtime::new();
    loop {
        print!("\n?> ");
        let _ = io::stdout().flush();

        // Read input from stdin
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read from standard input.");

        let res = runtime.parse_eval(input).unwrap();
        println!("=> {}", res);
    }
}
