use std::env;
use std::fs;
use std::process::exit;

use rust_compressor::{compress_rle, decompress_rle, compress_lz, decompress_lz};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: compress|decompress <input_file> <output_file> --rle|--lz");
        exit(1);
    }

    let action = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];
    let algorithm = &args[4];

    let input_data = fs::read(input_file).expect("Failed to read input file");

    let result = match algorithm.as_str() {
        "--rle" => {
            if action == "compress" {
                compress_rle(&input_data)
            } else {
                decompress_rle(&input_data)
            }
        }
        "--lz" => {
            if action == "compress" {
                compress_lz(&input_data)
            } else {
                decompress_lz(&input_data)
            }
        }
        _ => {
            eprintln!("Unknown algorithm: {}", algorithm);
            exit(1);
        }
    };

    fs::write(output_file, result).expect("Failed to write output file");
}

