use std::env;
use std::fs;
use std::process::exit;

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

fn compress_rle(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut count = 1;
    for i in 1..data.len() {
        if data[i] == data[i - 1] {
            count += 1;
        } else {
            result.push(data[i - 1]);
            result.push(count);
            count = 1;
        }
    }
    result.push(data[data.len() - 1]);
    result.push(count);
    result
}

fn decompress_rle(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    for i in 0..data.len() / 2 {
        let byte = data[2 * i];
        let count = data[2 * i + 1];
        result.extend(std::iter::repeat(byte).take(count as usize));
    }
    result
}

fn compress_lz(data: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let window_size = 20;
        let mut i = 0;
        
        while i < data.len() {
            let mut best_match_offset = 0;
            let mut best_match_length = 0;
            
            // Look back in the sliding window
            let start = if i > window_size { i - window_size } else { 0 };
            let end = i;
    
            // Find the longest match in the window
            for j in start..end {
                let mut length = 0;
                while i + length < data.len() && data[j + length] == data[i + length] {
                    length += 1;
                    if length == 255 { break; } // Limit match length to 255 bytes
                }
                if length > best_match_length {
                    best_match_offset = i - j;
                    best_match_length = length;
                }
            }
    
            // If a match was found
            if best_match_length >= 3 {  // We want to find matches at least 3 bytes long
                result.push(0x01);  // Match indicator
                result.push(best_match_offset as u8);  // Offset
                result.push(best_match_length as u8);  // Length
                i += best_match_length;  // Move ahead by the match length
            } else {
                // No match, add literal
                result.push(0x00);  // Literal indicator
                result.push(data[i]);  // Literal byte
                i += 1;  // Move ahead by 1 byte
            }
        }
        
        result
    }
    

fn decompress_lz(data: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        let mut i = 0;
        
        while i < data.len() {
            let command = data[i];
            i += 1;
            
            if command == 0x00 {
                // Literal byte
                result.push(data[i]);
                i += 1;
            } else if command == 0x01 {
                // Match: offset and length
                let offset = data[i] as usize;
                let length = data[i + 1] as usize;
                i += 2;
    
                // Copy from the sliding window (previously decompressed data)
                let start = result.len() - offset;
                for j in 0..length {
                    result.push(result[start + j]);
                }
            } else {
                // Invalid data (error handling can be added here)
                panic!("Invalid command in LZ77 stream");
            }
        }
        
        result
    }
    
