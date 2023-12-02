import readline from 'node:readline';

function makeMax(globalRegex) {
  return string => Math.max(...Array.from(string.matchAll(globalRegex), m => parseInt(m[1], 10)));
}

const maxRed = makeMax(/(\d+?) red/g);
const maxGreen = makeMax(/(\d+?) green/g);
const maxBlue = makeMax(/(\d+?) blue/g);

let sum = 0;

for await (const line of readline.createInterface({ input: process.stdin })) {
  sum += maxRed(line) * maxGreen(line) * maxBlue(line);
}

console.log(sum);
