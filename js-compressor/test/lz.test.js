const assert = require('assert');
const { compress, decompress } = require('../lz');

describe('LZ77 Compression', () => {
    it('should compress and decompress correctly', () => {
        const input = Buffer.from('ABABABABABAB');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });

    it('should handle literal bytes', () => {
        const input = Buffer.from('AABBAABBAABBA');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });
});
