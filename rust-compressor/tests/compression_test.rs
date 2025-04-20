use rust_compressor::{compress_rle, decompress_rle, compress_lz, decompress_lz};

#[test]
fn test_rle() {
    let input = b"aaabbbccccddddddeeeeeee";
    let compressed = compress_rle(input);
    let decompressed = decompress_rle(&compressed);
    assert_eq!(input.to_vec(), decompressed);
}

#[test]
fn test_lz() {
    let input = b"abcabcabcabcabcabc";
    let compressed = compress_lz(input);
    let decompressed = decompress_lz(&compressed);
    assert_eq!(input.to_vec(), decompressed);
}
