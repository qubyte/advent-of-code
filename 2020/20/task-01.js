'use strict';

const fs = require('fs');
const file = fs.readFileSync(process.argv[2], 'utf8').trim();

const tiles = {};

for (const rawTile of file.split('\n\n')) {
  const [firstLine, ...remainingLines] = rawTile.split('\n');
  const id = firstLine.match(/^Tile (\d+):$/)[1];
  const tile = remainingLines.map(line => line.split('').map(c => c === '#'));

  tiles[id] = tile;
}

function doSidesMatch(a, b) {
  return a.every((el, i) => el === b[i]);
}

function checkBoundaries(tileA, tileB) {
  const tileASides = [
    tileA[0],
    tileA[tileA.length - 1],
    tileA.map(r => r[0]),
    tileA.map(r => r[r.length - 1]),
    tileA[0].slice().reverse(),
    tileA[tileA.length - 1].slice().reverse(),
    tileA.map(r => r[0]).reverse(),
    tileA.map(r => r[r.length - 1]).reverse()
  ];
  const tileBSides = [
    tileB[0],
    tileB[tileB.length - 1],
    tileB.map(r => r[0]),
    tileB.map(r => r[r.length - 1])
  ];

  for (const sideA of tileASides) {
    for (const sideB of tileBSides) {
      if (doSidesMatch(sideA, sideB)) {
        return true;
      }
    }
  }

  return false;
}

const corners = [];

for (const [idA, tileA] of Object.entries(tiles)) {
  let matchingEdges = 0;

  for (const [idB, tileB] of Object.entries(tiles)) {
    if (idA === idB) {
      continue;
    }

    if (checkBoundaries(tileA, tileB)) {
      matchingEdges++;
    }
  }

  if (matchingEdges < 3) {
    corners.push(idA);
  }
}

console.log(corners.reduce((product, id) => product *= parseInt(id, 10), 1));
