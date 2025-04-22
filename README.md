# 🧰 Compression Tool

A command-line tool for compressing and decompressing files using **Run-Length Encoding (RLE)** and **LZ77** algorithms. Built with Rust and JavaScript, this tool is designed for efficiency and ease of use.

---

## 🚀 Features

- **Compression**: Reduce file sizes using RLE or LZ77 algorithms.
- **Decompression**: Restore original files from compressed formats.
- **Docker Support**: Run without installation using Docker.
- **Cross-Platform**: Compatible with Linux, macOS, and Windows.

---

## 📦 Installation

### Option 1: Using Docker (No Installation Required)

```bash
docker run -v $(pwd):/data rust-compressor compress /data/input.txt /data/output.cmp --rle
```

Replace `input.txt` with your file and `output.cmp` with your desired output file name.

### Option 2: Building from Source

1. Clone the repository:

   ```bash
   git clone https://github.com/Marc-Jr/compression-tool.git
   cd compression-tool
   ```

2. Build the Rust compressor:

   ```bash
   cd rust-compressor
   docker build -t rust-compressor .
   ```

3. Build the JavaScript compressor:

   ```bash
   cd ../js-compressor
   npm install
   ```

---

## 🛠️ Usage

### Compress a File

```bash
docker run -v $(pwd):/data rust-compressor compress /data/input.txt /data/output.cmp --rle
```

### Decompress a File

```bash
docker run -v $(pwd):/data rust-compressor decompress /data/output.cmp /data/output.txt --rle
```

---

## 🧪 Benchmarking

To test the performance of the compression and decompression:

1. Create a sample text file:

   ```bash
   echo "Sample text for compression testing." > test.txt
   ```

2. Run the benchmarking script:

   ```bash
   chmod +x benchmark.sh
   ./benchmark.sh
   ```

This will display the compression and decompression times and file sizes.

---


