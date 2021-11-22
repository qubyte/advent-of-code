'use strict';

const fs = require('fs');
const file = fs.readFileSync(process.argv[2], 'utf8').trim();

const tiles = new Map();

for (const rawTile of file.split('\n\n')) {
  const [firstLine, ...remainingLines] = rawTile.split('\n');
  const id = firstLine.match(/^Tile (\d+):$/)[1];
  const tile = remainingLines.map(line => line.split('').map(c => c === '#'));

  tiles.set(id, tile);
}

console.log(tiles.size)

function rotate(tile) {
  const rotated = [];

  for (let i = 0; i < tile[0].length; i++) {
    rotated.push(tile.map(r => r[i]).reverse());
  }

  return rotated;
}

function flipHorizontal(tile) {
  for (const row of tile) {
    row.reverse();
  }
}

function flipVertical(tile) {
  tile.reverse();
}

function doSidesMatch(a, b) {
  return a.every((el, i) => el === b[i]);
}

function checkBoundaries(tileA, tileB) {
  const tileASides = [
    tileA[0],
    tileA[tileA.length - 1],
    tileA.map(r => r[0]),
    tileA.map(r => r[r.length - 1])
  ];
  const tileBSides = [
    tileB[tileB.length - 1], // bottom
    tileB[tileB.length - 1].slice().reverse(), // bottom, twice rotated

    tileB.map(r => r[r.length - 1]),
    tileB.map(r => r[r.length - 1]).reverse(),

    tileB[0],
    tileB[0].slice().reverse(),

    tileB.map(r => r[0]),
    tileB.map(r => r[0]).reverse()
  ];

  for (const [iA, sideA] of tileASides.entries()) {
    for (const [iB, sideB] of tileBSides.entries()) {
      if (doSidesMatch(sideA, sideB)) {
        return [iA, iB];
      }
    }
  }

  return false;
}

const edges = new Map();

for (const [idA, tileA] of tiles) {
  const edgeSet = {};

  edges.set(idA, edgeSet);

  for (const [idB, tileB] of tiles) {
    if (idA === idB) {
      continue;
    }

    const transforms = checkBoundaries(tileA, tileB);

    if (transforms) {
      edgeSet[idB] = transforms;
    }
  }
}

// Get a corner
const [id, adjoining] = Array.from(edges.entries()).find(([_, e]) => Object.keys(e).length === 2);

const tile = tiles.get(id);
const [adjoiningA, adjoiningB] = Object.entries(adjoining);

if (adjoiningA[1][0] === 0 || adjoiningB[1][0] === 0) {
  flipVertical(tile);
}
if (adjoiningA[1][0] === 2 || adjoiningB[1][0] === 2) {
  flipHorizontal(tile);
}

function formatTile(tile) {
  return tile.map(row => row.map(el => el ? '#' : '.').join('')).join('\n');
}

const assembled = [[id]];
const used = new Set();

used.add(id);

while (true) {
  const currentRow = assembled[assembled.length - 1];
  const isNewRow = currentRow.length === 0;
  const laidTileId = isNewRow ? assembled[assembled.length - 2][0] : currentRow[currentRow.length - 1];
  const laidTile = tiles.get(laidTileId);
  const neighbourIds = Object.keys(edges.get(laidTileId)).filter(id => !used.has(id));
  const sideToMatch = isNewRow ? laidTile[laidTile.length - 1] : laidTile.map(r => r[r.length - 1]);

  let matchingId;

  outer: for (const id of neighbourIds) {
    let neighbourTile = tiles.get(id);

    for (let i = 0; i < 4; i++) {
      const left = neighbourTile.map(r => r[0]);

      if (doSidesMatch(sideToMatch, left)) {
        matchingId = id;
        tiles.set(id, neighbourTile);
        break outer;
      }

      left.reverse();

      if (doSidesMatch(sideToMatch, left)) {
        flipVertical(neighbourTile);
        matchingId = id;
        tiles.set(id, neighbourTile);
        break outer;
      }

      neighbourTile = rotate(neighbourTile);
    }
  }

  currentRow.push(matchingId);
  used.add(matchingId);

  if (isNewRow) {
    const t = rotate(tiles.get(matchingId));
    flipHorizontal(t);
    tiles.set(matchingId, t);
  } else if (used.size === tiles.size) {
    break;
  } else if (Object.keys(edges.get(currentRow[0])).length === Object.keys(edges.get(matchingId)).length) {
    assembled.push([]);
  }
}

const trimmedGrid = assembled.map(row => {
  return row.map(id => {
    const tile = tiles.get(id);

    return tile.slice(1, -1).map(tileRow => tileRow.slice(1, -1));
  });
})

let unwrapped = [];

for (const row of trimmedGrid) {
  const depth = row[0].length;

  for (let i = 0; i < depth; i++) {
    const unwrappedRow = [];

    for (const tile of row) {
      unwrappedRow.push(...tile[i]);
    }

    unwrapped.push(unwrappedRow);
  }
}

function countSeaMonsters(grid) {
  const flattened = grid.map(row => row.map(el => el ? '#' : '.').join(''));

  let snakes = 0;

  for (let i = 1; i < flattened.length - 2; i++) {
    const matches = flattened[i].matchAll(/#....##....##....###/g);

    for (const { index } of matches) {
      if (flattened[i - 1][index + 18] !== '#') {
        console.log('NO HEAD')
        continue;
      }
      if (!flattened[i + 1].slice(index, index + 17).match(/^.#..#..#..#..#..#$/)) {
        console.log('NO LOWER')
        continue;
      }

      snakes++;
    }
  }

  return snakes;
}

unwrapped = rotate(unwrapped);

const count = countSeaMonsters(unwrapped);

const total = unwrapped.reduce((total, row) => total + row.reduce((total, el) => {
  return total + (el ? 1 : 0);
}, 0), - 15 * count);

console.log(total)
