function compress(data) {
  let result = [];
  let count = 1;
  for (let i = 1; i < data.length; i++) {
      if (data[i] === data[i - 1]) {
          count++;
      } else {
          result.push(data[i - 1], count);
          count = 1;
      }
  }
  result.push(data[data.length - 1], count);
  return Buffer.from(result);
}

function decompress(data) {
  let result = [];
  for (let i = 0; i < data.length; i += 2) {
      let byte = data[i];
      let count = data[i + 1];
      result.push(...Array(count).fill(byte));
  }
  return Buffer.from(result);
}

module.exports = { compress, decompress };
