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

const flipped = new Set();

for (const direction of directions) {
  const coords = [0, 0, 0];

  for (const step of direction) {
    switch (step) {
    case 'e':
      coords[0]++;
      coords[1]--;
      continue;
    case 'w':
      coords[0]--;
      coords[1]++;
      continue;
    case 'ne':
      coords[0]++;
      coords[2]--;
      continue;
    case 'sw':
      coords[0]--;
      coords[2]++;
      continue;
    case 'nw':
      coords[1]++;
      coords[2]--;
      continue;
    case 'se':
      coords[1]--;
      coords[2]++;
      continue;
    }
  }

  const key = coords.join(',');

  if (flipped.has(key)) {
    flipped.delete(key);
  } else {
    flipped.add(key);
  }
}

console.log(flipped.size);
