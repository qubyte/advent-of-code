import readline from 'node:readline';

const rl = readline.createInterface({ input: process.stdin });
const gameNumberRegex = /^Game (\d+?):/;
const redRegex = /(\d+?) red/g;
const greenRegex = /(\d+?) green/g;
const blueRegex = /(\d+?) blue/g;

const MAX_RED = 12;
const MAX_GREEN = 13;
const MAX_BLUE = 14;

let sum = 0;

for await (const line of rl) {
  const gameNumberMatch = line.match(gameNumberRegex);
  const gameNumber = gameNumberMatch && parseInt(gameNumberMatch[1], 10)

  if (isNaN(gameNumber)) {
    continue;
  }

  const redMatches = Array.from(line.matchAll(redRegex), m => parseInt(m, 10));
  const greenMatches = Array.from(line.matchAll(greenRegex), m => parseInt(m, 10));
  const blueMatches = Array.from(line.matchAll(blueRegex), m => parseInt(m, 10));

  const maxRed = Math.max(...redMatches)
  const maxGreen = Math.max(...greenMatches)
  const maxBlue = Math.max(...blueMatches)

  if (maxRed <= MAX_RED && maxGreen <= MAX_GREEN && maxBlue <= MAX_BLUE) {
    sum += gameNumber;
  }
}

console.log(sum);
