'use strict';

const fs = require('fs');
const file = fs.readFileSync(process.argv[2], 'utf8').trim();
const lines = file.split('\n');

const ingredients = new Map();
const allergens = new Map();

for (const line of lines) {
  const [rawIngredients, rawAllergens] = line.split('(contains ');
  const separatedIngredients = rawIngredients.trim().split(' ');
  const separatedAllergens = rawAllergens.trim().slice(0, -1).split(', ').filter(s => s);

  for (const allergen of separatedAllergens) {
    if (allergens.has(allergen)) {
      const allergenIngredients = allergens.get(allergen);

      for (const allergenIngredient of allergenIngredients) {
        if (!separatedIngredients.includes(allergenIngredient)) {
          allergenIngredients.delete(allergenIngredient);
        }
      }
    } else {
      const ingredientsForAllergen = new Set(separatedIngredients);

      for (const ingredient of ingredients) {
        ingredientsForAllergen.delete(ingredient);
      }

      allergens.set(allergen, ingredientsForAllergen);
    }
  }

  for (const i of separatedIngredients) {
    ingredients.set(i, (ingredients.get(i) || 0) + 1);
  }
}

const resolvedAllergens = new Map();

while (allergens.size) {
  for (const [name, possibleIngredients] of allergens) {
    const filteredIngredients = Array.from(possibleIngredients)
      .filter(n => !Array.from(resolvedAllergens.values()).includes(n));

    if (filteredIngredients.length === 1) {
      resolvedAllergens.set(name, filteredIngredients[0]);
      allergens.delete(name);
    }
  }
}

const names = Array.from(resolvedAllergens.keys()).sort();
const allergenList = names.map(n => resolvedAllergens.get(n)).join(',');

console.log(allergenList);
