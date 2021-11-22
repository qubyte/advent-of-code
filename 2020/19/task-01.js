'use strict';

const fs = require('fs');
const file = fs.readFileSync(process.argv[2], 'utf8');
const [encodedRules, messages] = file.trim().split('\n\n').map(s => s.split('\n'));
const rules = [];
let aIndex = -1;
let bIndex = -1;

for (const encodedRule of encodedRules) {
  const [index, encodedRuleList] = encodedRule.split(': ');

  if (encodedRuleList === '"a"') {
    rules[index] = [encodedRuleList[1]];
    aIndex = Number(index);
  } else if (encodedRuleList === '"b"') {
    rules[index] = [encodedRuleList[1]];
    bIndex = Number(index);
  } else {
    rules[index] = encodedRuleList.split(' | ').map(indices => indices.split(' ').map(Number));
  }
}

const toSubstitute = new Set([aIndex, bIndex]);
const completed = [];

function substitute() {
  for (const index of toSubstitute) {
    const replacement = rules[index];

    for (const rule of rules) {
      if (rule === replacement) {
        continue;
      }

      for (let i = rule.length - 1; i >= 0; i--) {
        const option = rule[i];

        if (typeof option === 'string') {
          continue;
        }

        let indexInOption = option.indexOf(index);

        if (indexInOption == -1) {
          continue;
        }

        const expanded = replacement.map(r => {
          const copy = option.slice();
          copy.splice(indexInOption, 1, ...r);
          return copy;
        });

        rule.splice(i, 1, ...expanded);
      }
    }
  }

  collect();
}

function collect() {
  for (let i = 0; i < rules.length; i++) {
    if (completed.includes(i)) {
      continue;
    }

    const rule = rules[i];

    let isComplete = true;

    for (let j = 0; j < rule.length; j++) {
      if (typeof rule[j] === 'string') {
        continue;
      }

      if (rule[j].every(el => typeof el === 'string')) {
        rule[j] = rule[j].join('');
      } else {
        isComplete = false;
      }
    }

    if (isComplete) {
      completed.push(i);
      toSubstitute.add(i);
    }
  }

  if (completed.length !== rules.length) {
    substitute();
  }
}

substitute();

let matches = 0;

for (const message of messages) {
  if (rules[0].includes(message)) {
    matches++;
  }
}

console.log(matches);
