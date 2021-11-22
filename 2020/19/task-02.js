'use strict';

const fs = require('fs');
const file = fs.readFileSync(process.argv[2], 'utf8');
const [encodedRules, messages] = file.trim().split('\n\n').map(s => s.split('\n'));
let rules = [];
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

// No need to calculate these, we build them later.
rules[8] = ['x'];
rules[11] = ['y'];

const toSubstitute = new Set([aIndex, bIndex]);
const completed = [];

function ruleIsKnown(index) {
  return rules[index].every(option => typeof option === 'string');
}

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
}

function collect() {
  for (let i = 0; i < rules.length; i++) {
    if (completed.includes(i)) {
      continue;
    }

    const rule = rules[i];

    let isComplete = true;

    for (let j = rule.length - 1; j >= 0; j--) {
      if (typeof rule[j] === 'string') {
        continue;
      }

      if (rule[j].every(el => typeof el === 'string')) {
        rule[j] = rule[j].join('');
        if (!messages.some(message => message.includes(rule[j]))) {
          rule.splice(j, 1);
        }
      } else {
        isComplete = false;
      }
    }

    if (isComplete) {
      completed.push(i);

      if (rule.length) {
        toSubstitute.add(i);
      }
    }
  }
}

while (completed.length !== rules.length) {
  collect();
  substitute();
}

// Now we have rules 8 and 11, we can compose them recursively until we have
// a set of rule 0 options long enough. That's too memory intensive though, so
// instead we go through the messages one by one and check that blocks of 8
// characters match a pattern. Given the rules:

// rules[8] = [[42], [42, 8]];
// rules[11] = [[42, 31], [42, 11, 31]];
// rules[0] = [[8, 11]]

// rule 0 must be composed by options from rule 42, followed by options of rule
// 31. Once a rule 31 option is encountered, all following blocks of 8 must be
// options from rule 31. i.e.:

// rule[0] => [[42 42 31], [42 42 42 31], [42 42 42 42 31], [42 42 42 31 31], ]

let total = 0;

outer: for (const message of messages) {
  // At least one option from rule 42 must appear first.
  const max31count = Math.floor((message.length - 8) / 16);

  let count31 = 0;

  for (let i = 0; i < message.length; i += 8) {
    const substring = message.slice(i, i + 8);

    if (!count31 && rules[42].includes(substring)) {
      continue;
    } else if (rules[31].includes(substring)) {
      count31++;
      continue;
    } else {
      continue outer;
    }
  }

  // At least one rule 31 must appear last.
  if (count31 > 0 && count31 <= max31count) {
    total++;
  }
}

console.log(total);
