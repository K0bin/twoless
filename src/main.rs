use std::{fs::File, io::BufWriter};

use triplet::*;

mod triplet;
mod sat;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        println!("Usage: twoless n k filename.cnf");
        return;
    }

    let n_parsed = args[1].parse::<u8>();
    if n_parsed.is_err() {
        println!("Parameter n has to be a number!");
        return;
    }
    let n: u8 = n_parsed.unwrap();

    let k_parsed = args[2].parse::<u8>();
    if k_parsed.is_err() {
        println!("Parameter k has to be a number!");
        return;
    }
    let k: u8 = k_parsed.unwrap();

    let filename = args[3].clone();
    if filename.is_empty() {
        println!("Please specify a filename!");
        return;
    }

    let file = File::create(&filename);
    if file.is_err() {
        println!("Failed to open file {}!", &filename);
        return;
    }
    let file = file.unwrap();
    let mut writer = BufWriter::new(file);
    
    println!("Generating SAT for n={}, k={}", n, k);
    let set = TripletSet::new(n);
    let sequence = TripletSequence::new(set, k);
    let sat = sequence.generate_sat();
    sat.write_to_file(&mut writer);
    println!("Done!");
}
