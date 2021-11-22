'use strict';

const fs = require('fs');
const file = fs.readFileSync(process.argv[2], 'utf8').trim();

const [player1, player2] = file.split('\n\n').map(rawDeck => {
  return rawDeck.trim().split('\n').slice(1).map(line => parseInt(line, 10));
});

let rounds = 0;

while (player1.length && player2.length) {
  rounds++;

  const card1 = player1.shift();
  const card2 = player2.shift();

  if (card1 > card2) {
    player1.push(card1, card2);
  } else {
    player2.push(card2, card1);
  }
}

let winner;

if (player1.length) {
  console.log('Player 1 wins after', rounds, 'rounds.');
  winner = player1;
} else {
  console.log('Player 2 wins after', rounds, 'rounds.');
  winner = player2;
}

const score = winner.reduce((score, card, i) => {
  return score + card * (winner.length - i);
}, 0);

console.log(score)
