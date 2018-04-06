extern crate docopt;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use docopt::Docopt;
use std::env;
use std::io::Read;
use std::fs::File;
use std::time::Instant;

mod pascal;

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(env::args()).deserialize())
        .unwrap_or_else(|e| e.exit());

    for input in &args.arg_inputs {
        let mut s = String::new();
        if let Err(err) = File::open(input).and_then(|mut f| f.read_to_string(&mut s)) {
            println!("Input `{}`: I/O Error {}", input, err);
            continue;
        }

        let time_stamp = Instant::now();
        let result = pascal::fileParser::new().parse(&s);
        let elapsed = time_stamp.elapsed();
        let elapsed = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 / 1000_000_000.0;

        match result {
            Ok(()) => println!("Input `{}` ({}s): OK", input, elapsed),
            Err(err) => println!("Input `{}` ({}s): parse error {:?}", input, elapsed, err),
        }
    }
}

const USAGE: &'static str = "
Usage: pascal <inputs>...

Parses each input file.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_inputs: Vec<String>,
}
