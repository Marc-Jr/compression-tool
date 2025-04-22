#!/bin/bash

# File paths
SOURCE_FILE="test.txt"
COMPRESSED_FILE="test.txt.cmp"
DECOMPRESSED_FILE="test.txt.dec"

# Clean up old files
rm -f $COMPRESSED_FILE $DECOMPRESSED_FILE compression_time.txt decompression_time.txt

# Compress and record time
{ time docker run -v $(pwd):/data rust-compressor compress /data/$SOURCE_FILE /data/$COMPRESSED_FILE --rle; } 2> compression_time.txt

# Decompress and record time
{ time docker run -v $(pwd):/data rust-compressor decompress /data/$COMPRESSED_FILE /data/$DECOMPRESSED_FILE --rle; } 2> decompression_time.txt

# Extract timing
compress_time=$(grep real compression_time.txt | awk '{print $2}')
decompress_time=$(grep real decompression_time.txt | awk '{print $2}')

# Extract sizes
size_original=$(du -h $SOURCE_FILE | cut -f1)
size_compressed=$(du -h $COMPRESSED_FILE | cut -f1)
size_decompressed=$(du -h $DECOMPRESSED_FILE | cut -f1)

# Print table
echo ""
echo "📊 Benchmark Results:"
echo ""
printf "%-25s | %-10s\n" "Metric" "Result"
printf "%-25s | %-10s\n" "-------------------------" "----------"
printf "%-25s | %-10s\n" "Compression Time" "$compress_time"
printf "%-25s | %-10s\n" "Decompression Time" "$decompress_time"
printf "%-25s | %-10s\n" "Original File Size" "$size_original"
printf "%-25s | %-10s\n" "Compressed Size" "$size_compressed"
printf "%-25s | %-10s\n" "Decompressed Size" "$size_decompressed"
