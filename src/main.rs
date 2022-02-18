use clap::Parser;
use sha2::{Sha256, Digest};
use std::time::{Duration, Instant};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let _args = Args::parse();

    let hashcnt = 10_000_000;

    let genesis_hash = "GENESIS_HASH_RUST_UNICORN";

    let mut data = String::from("");

    let tick_size = 100_000;

    let start = Instant::now();

    for cnt in 0 .. hashcnt {
        let mut sha256 = Sha256::new();

        if cnt == 0 {
            sha256.update(genesis_hash);

        } else {
            sha256.update(data);

        }

        let d: String = format!("{:X}", sha256.finalize());

        if cnt % tick_size == 1 {
            print!("{} => {}\n", cnt-1, d);

        }

        data = d;
    
    }

    let duration = start.elapsed();

    println!("{:?} hashes took - duration {:?}\n", hashcnt, duration);


}