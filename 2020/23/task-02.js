'use strict';

const rawCups = process.argv[2].split('').map(Number);

const cups = new Map();
const max = 1e6;
const rounds = 10e6;

// Build a linked list of cups.
for (let i = 0; i < max; i++) {
  const label = rawCups[i] || i + 1;
  const previous = rawCups[i - 1] || i;
  const next = rawCups[i + 1] || i + 2;

  cups.set(label, { label, previous, next });
}

// This is the target cup. Each iteration will update it.
let cupt = cups.get(rawCups[0]);

// Connect last card to first.
cupt.previous = max;
cups.get(max).next = cupt.label;

for (let i = 0; i < rounds; i++) {
  // We're (re)moving these.
  const cup1 = cups.get(cupt.next);
  const cup2 = cups.get(cup1.next);
  const cup3 = cups.get(cup2.next);

  // This is the cup following the removed cups, which will be the next target.
  const cup4 = cups.get(cup3.next);

  const cupsRemoved = [cup1.label, cup2.label, cup3.label];

  // Complete the removal of the three cups by wiring the cup next of cup3 to
  // the target as its next.
  cupt.next = cup4.label;
  cup4.previous = cupt.label;

  // Get the destination cup.
  let destination = cupt.label;

  do {
    destination = (destination - 1) || max;
  } while (cupsRemoved.includes(destination));

  const cupd = cups.get(destination);
  const cupn = cups.get(cupd.next);

  // Splice the removed cups clockwise of the destination cup.
  cupd.next = cup1.label;
  cup1.previous = cupd.label;
  cup3.next = cupn.label;
  cupn.previous = cup3.label;

  cupt = cup4;
}

const cup1 = cups.get(1).next;
const cup2 = cups.get(cup1).next;

console.log(cup1 * cup2);
