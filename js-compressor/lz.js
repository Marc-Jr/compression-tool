function compress(data) {
  let result = [];
  const windowSize = 256;  // Size of the sliding window (arbitrary limit)
  let i = 0;

  while (i < data.length) {
      let bestMatchOffset = 0;
      let bestMatchLength = 0;

      // Look back in the sliding window
      const start = Math.max(0, i - windowSize);
      const end = i;

      // Find the longest match in the window
      for (let j = start; j < end; j++) {
          let length = 0;

          // Compare data starting from j and i
          while (i + length < data.length && data[j + length] === data[i + length] && length < 255) {
              length++;
          }

          if (length > bestMatchLength) {
              bestMatchOffset = i - j;
              bestMatchLength = length;
          }
      }

      if (bestMatchLength >= 3) {  // Match found
          result.push(0x01);  // Indicate a match
          result.push(bestMatchOffset);  // Offset
          result.push(bestMatchLength);  // Length
          i += bestMatchLength;  // Move ahead by the match length
      } else {
          result.push(0x00);  // Indicate a literal byte
          result.push(data[i]);  // Literal byte
          i++;  // Move ahead by 1 byte
      }
  }

  return Buffer.from(result);
}

function decompress(data) {
  let result = [];
  let i = 0;

  while (i < data.length) {
      const command = data[i];
      i++;

      if (command === 0x00) {
          // Literal byte
          result.push(data[i]);
          i++;
      } else if (command === 0x01) {
          // Match: offset and length
          const offset = data[i];
          const length = data[i + 1];
          i += 2;

          // Copy from the sliding window (previously decompressed data)
          let start = result.length - offset;
          for (let j = 0; j < length; j++) {
              result.push(result[start + j]);
          }
      } else {
          throw new Error("Invalid command in LZ77 stream");
      }
  }

  return Buffer.from(result);
}

module.exports = { compress, decompress };
