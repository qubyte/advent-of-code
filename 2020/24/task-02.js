'use strict';

const fs = require('fs');
const file = fs.readFileSync(process.argv[2], 'utf8').trim();
const directions = file.split('\n').map(line => {
  let str = line.trim();

  const parsed = [];

  while (str.length) {
    if (str[0] === 'e' || str[0] === 'w') {
      parsed.push(str[0]);
      str = str.slice(1);
    } else {
      parsed.push(str[0] + str[1]);
      str = str.slice(2);
    }
  }

  return parsed;
});

// Hexagonal grids of this orientation can have cubic coordinates:
//  e => (x++, y--, z)
//  w => (x--, y++, z)
// ne => (x++. y, z--)
// sw => (x--. y, z++)
// nw => (x, y++, z--)
// se => (x, y--, z++)

let flipped = new Set();

let maxX = 0;
let minX = 0;
let maxY = 0;
let minY = 0;
let maxZ = 0;
let minZ = 0;

for (const direction of directions) {
  let x = 0;
  let y = 0;
  let z = 0;

  for (const step of direction) {
    switch (step) {
    case 'e':
      x++;
      y--;
      continue;
    case 'w':
      x--;
      y++;
      continue;
    case 'ne':
      x++;
      z--;
      continue;
    case 'sw':
      x--;
      z++;
      continue;
    case 'nw':
      y++;
      z--;
      continue;
    case 'se':
      y--;
      z++;
      continue;
    }
  }

  maxX = Math.max(maxX, x + 1);
  maxY = Math.max(maxY, y + 1);
  maxZ = Math.max(maxZ, z + 1);
  minX = Math.min(minX, x - 1);
  minY = Math.min(minY, y - 1);
  minZ = Math.min(minZ, z - 1);

  const key = [x, y, z].join(',');

  if (flipped.has(key)) {
    flipped.delete(key);
  } else {
    flipped.add(key);
  }
}

// The furthest in any direction a flipped tile can be is the length of the
// longest line + i, where i is the iteration.
for (let i = 0; i < 100; i++) {
  let next = new Set();

  let nextMaxX = 0;
  let nextMaxY = 0;
  let nextMaxZ = 0;
  let nextMinX = 0;
  let nextMinY = 0;
  let nextMinZ = 0;

  for (let x = minX; x <= maxX; x++) {
    for (let y = minY; y <= maxY; y++) {
      for (let z = minZ; z <= maxZ; z++) {
        const neighbours = [
          [x + 1, y - 1, z].join(','), // e
          [x - 1, y + 1, z].join(','), // w
          [x + 1, y, z - 1].join(','), // ne
          [x - 1, y, z + 1].join(','), // sw
          [x, y + 1, z - 1].join(','), // nw
          [x, y - 1, z + 1].join(',')  // se
        ]

        const count = neighbours.reduce((total, key) => {
          return flipped.has(key) ? total + 1 : total;
        }, 0);

        const coord = [x, y, z].join(',');

        if (count === 2 || (count === 1 && flipped.has(coord))) {
          next.add(coord);
          nextMaxX = Math.max(nextMaxX, x + 1);
          nextMaxY = Math.max(nextMaxY, y + 1);
          nextMaxZ = Math.max(nextMaxZ, z + 1);
          nextMinX = Math.min(nextMinX, x - 1);
          nextMinY = Math.min(nextMinY, y - 1);
          nextMinZ = Math.min(nextMinZ, z - 1);
        }
      }
    }
  }

  maxX = nextMaxX;
  maxY = nextMaxY;
  maxZ = nextMaxZ;
  minX = nextMinX;
  minY = nextMinY;
  minZ = nextMinZ;

  flipped = next;
}

console.log(flipped.size);
