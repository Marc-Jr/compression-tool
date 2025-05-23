use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process::exit;

use rust_compressor::{compress_rle, decompress_rle, compress_lz, decompress_lz};

fn main() {
    let args: Vec<String> = env::args().collect();

    // stdin/stdout mode: 3 args = action + algorithm (e.g., compress --rle)
    if args.len() == 3 {
        let action = &args[1];
        let algorithm = &args[2];

    
        let mut input_data = Vec::new();
        io::stdin().read_to_end(&mut input_data).expect("Failed to read from stdin");

    
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

    
        io::stdout().write_all(&result).expect("Failed to write to stdout");
        return;
    }

    // file mode: 5 args = action input_file output_file algorithm
    if args.len() == 5 {
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
        return;
    }

    // Invalid usage
    eprintln!("Usage:");
    eprintln!("  rust-compressor compress <input> <output> --rle|--lz");
    eprintln!("  rust-compressor decompress <input> <output> --rle|--lz");
    eprintln!("  echo 'text' | rust-compressor compress --rle > out.cmp");
    exit(1);
}
