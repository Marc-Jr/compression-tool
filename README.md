
# Compression Tool

A command-line utility for compressing and decompressing files using **Run-Length Encoding (RLE)** and **LZ77** algorithms. Built with **Rust** and **JavaScript**, this tool is designed for efficiency, cross-platform use, and educational clarity.

---

## Features

- **Compression & Decompression**: Supports RLE and LZ77.
- **Dual Implementation**: Built in both Rust (performance) and JavaScript (portability).
- **Docker Support**: No local setup needed—just run in a container.
- **Cross-Platform**: Works on Linux, macOS, and Windows.

---

## Installation

### Option 1: Use Docker (No Rust or Node setup required)

```bash
docker run -v $(pwd):/data rust-compressor compress /data/input.txt /data/output.cmp --rle
```

### Option 2: Build from Source

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

3. Set up the JavaScript compressor:

   ```bash
   cd ../js-compressor
   npm install
   ```

---

## Usage

### Compress a File

#### 🐳 Using Docker (Rust):

```bash
docker run -v $(pwd):/data rust-compressor compress /data/input.txt /data/output.cmp --rle
```

#### 🦀 Using Rust Locally:

```bash
# In the rust-compressor directory
cargo run -- compress input.txt output.cmp --rle
```

#### Using JavaScript (Node.js):

```bash
# In the js-compressor directory
node compress.js input.txt output.cmp --rle
```

---

### Decompress a File

#### 🐳 Using Docker (Rust):

```bash
docker run -v $(pwd):/data rust-compressor decompress /data/output.cmp /data/output.txt --rle
```

#### 🦀 Using Rust Locally:

```bash
cargo run -- decompress output.cmp output.txt --rle
```

#### Using JavaScript (Node.js):

```bash
node decompress.js output.cmp output.txt --rle
```

---

## Benchmarking

To test compression speed and effectiveness:

1. Create a test file:

   ```bash
   echo "Sample text for compression testing." > test.txt
   ```

2. Run the benchmarking script:

   ```bash
   chmod +x benchmark.sh
   ./benchmark.sh
   ```

---

## 📚 Learning Resources

- [Run-Length Encoding (RLE)](https://en.wikipedia.org/wiki/Run-length_encoding)
- [LZ77 Compression Algorithm](https://en.wikipedia.org/wiki/LZ77_and_LZ78_algorithms)

---

## 🤝 Contributing

Open to improvements! Fork the repo, submit issues, or create a pull request.

---

