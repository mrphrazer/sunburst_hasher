use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::read_to_string;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Config {
    #[structopt(long = "min", help = "Minimum string length")]
    pub min_depth: usize,
    #[structopt(long = "max", help = "Maximum string length")]
    pub max_depth: usize,
    #[structopt(long = "input", help = "File with hash values")]
    pub input_file: String,
}

fn get_hash(s: &[u8]) -> u64 {
    let mut num: u64 = 14695981039346656037;
    for b in s {
        num ^= *b as u64;
        num = num.wrapping_mul(1099511628211);
    }
    num ^ 0x5BAC903BA7D81967
}
fn main() {
    let config = Config::from_args();

    let hashes: HashSet<u64> = read_to_string(config.input_file)
        .expect("Could not read file")
        .split('\n')
        .map(|s| s.parse::<u64>().expect("Could not parse convert to u64."))
        .collect();

    let alphabet: Vec<u8> = "abcdefghijklmnopqrstuvwxyz0123456789-_."
        .as_bytes()
        .to_vec();

    for n in config.min_depth..=config.max_depth {
        println!("n = {}:", n);
        (0..alphabet.len().pow(n as u32))
            .into_par_iter()
            .for_each(|mut ops| {
                let mut bytes: Vec<u8> = vec![0; n as usize];
                for index in 0..n {
                    bytes[index as usize] = alphabet[ops % alphabet.len()] as u8;
                    ops /= alphabet.len();
                }

                let h = get_hash(&bytes[..]);
                if hashes.contains(&h) {
                    let s = std::str::from_utf8(&bytes[..]).unwrap();
                    println!("{} -- {}", s, h);
                }
            });
        println!("\n\n");
    }
}
