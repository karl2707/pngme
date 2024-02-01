mod commands;

use std::time::Instant;
use clap::Parser;
use crate::commands::Args;

fn main() {
    let start = Instant::now();
    let args = Args::parse();

    let result = match args {
        Args::Encode(args) => args.encode(),
        Args::Decode(args) => args.decode(),
        Args::Remove(args) => args.remove(),
        Args::Print(args) => args.print()
    };

    match result {
        Ok(_) => println!("Program exited successfully."),
        Err(e) => println!("Program terminated unsuccessfully: {}.", e),
    }

    let duration = start.elapsed();
    println!("Execution took {:?} seconds", duration);
}
