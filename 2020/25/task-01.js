'use strict';

const fs = require('fs');
const file = fs.readFileSync(process.argv[2], 'utf8').trim();
const [cardPublicKey, doorPublicKey] = file.split('\n').map(Number);

let cardLoopSize = 0;
let doorLoopSize = 0;
let value = 1;

while (value !== cardPublicKey) {
  cardLoopSize++;
  value = (value * 7) % 20201227;
}

console.log('card loop size:', cardLoopSize);

value = 1;

while (value !== doorPublicKey) {
  doorLoopSize++;
  value = (value * 7) % 20201227;
}

console.log('door loop size:', doorLoopSize);

let encryptionKey = 1;

for (let i = 0; i < cardLoopSize; i++) {
  encryptionKey = (encryptionKey * doorPublicKey) % 20201227;
}

console.log('encryption key:', encryptionKey);
