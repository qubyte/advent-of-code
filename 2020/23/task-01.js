'use strict';

const cups = process.argv[2].split('').map(Number);

function move(index) {
  const target = cups[index];
  const removed = cups.splice(index + 1, 3);
  const toRemoveFromStart = 3 - removed.length;

  removed.push(...cups.splice(0, toRemoveFromStart));

  const min = Math.min(...cups);
  const max = Math.max(...cups);

  let destination = target - 1;
  let destinationIndex = cups.indexOf(destination);

  while (destinationIndex === -1) {
    destination = destination <= min ? max : destination - 1;
    destinationIndex = cups.indexOf(destination);
  }

  cups.splice(destinationIndex + 1, 0, ...removed);

  return (destinationIndex < index) ? index + 3 - toRemoveFromStart : index;
}

let targetIndex = 0;

for (let i = 0; i < 100; i++) {
  targetIndex = (move(targetIndex) + 1) % cups.length;
}

const index1 = cups.indexOf(1);

console.log(cups.slice(index1 + 1).concat(cups.slice(0, index1)).join(''));
