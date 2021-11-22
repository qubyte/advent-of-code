'use strict';

const fs = require('fs');
const file = fs.readFileSync(process.argv[2], 'utf8').trim();

const [player1, player2] = file.split('\n\n').map(rawDeck => {
  return rawDeck.trim().split('\n').slice(1).map(line => parseInt(line, 10));
});

function combat(player1, player2) {
  const previousRounds = new Set();

  while (player1.length && player2.length) {
    const roundKey = player1.join(',') + ':' + player2.join(',');

    if (previousRounds.has(roundKey)) {
      return player1;
    }

    previousRounds.add(roundKey);

    const card1 = player1.shift();
    const card2 = player2.shift();

    if (card1 <= player1.length && card2 <= player2.length) {
      const subPlayer1 = player1.slice(0, card1);
      const subPlayer2 = player2.slice(0, card2);

      if (combat(subPlayer1, subPlayer2) === subPlayer1) {
        player1.push(card1, card2);
      } else {
        player2.push(card2, card1);
      }
    } else if (card1 > card2) {
      player1.push(card1, card2);
    } else {
      player2.push(card2, card1);
    }
  }


  return player1.length ? player1 : player2;
}

const winner = combat(player1, player2);

const score = winner.reduce((score, card, i) => {
  return score + card * (winner.length - i);
}, 0);

console.log(score)
