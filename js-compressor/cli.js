const fs = require('fs');
const { compressRLE, decompressRLE } = require('./rle');
const { compressLZ, decompressLZ } = require('./lz');

const args = process.argv.slice(2);

if (args.length !== 4) {
  console.error('Usage: compress|decompress <input_file> <output_file> --rle|--lz');
  process.exit(1);
}

const [action, inputFile, outputFile, method] = args;
const input = fs.readFileSync(inputFile);

let output;

if (method === '--rle') {
  output = action === 'compress' ? compressRLE(input) : decompressRLE(input);
} else if (method === '--lz') {
  output = action === 'compress' ? compressLZ(input) : decompressLZ(input);
} else {
  console.error('Unknown method:', method);
  process.exit(1);
}

fs.writeFileSync(outputFile, output);
console.log(`${action}ion with ${method.slice(2).toUpperCase()} completed.`);
