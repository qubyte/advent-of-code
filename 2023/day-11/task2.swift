struct Galaxy: Hashable {
  var x: Int
  let y: Int
}

// The universe is an array of galaxy coordinates. No need to keep
// track of the space between them.
var universe: [Galaxy] = []
var colsWithGalaxies: Set<Int> = Set()
var row = 0
var cols = 0

let expansion = 1000000

while let line = readLine() {
  cols = line.count

  var hasGalaxies = false

  for (i, char) in line.enumerated() {
    if char == "#" {
      hasGalaxies = true
      universe.append(Galaxy(x: i, y: row))
      colsWithGalaxies.insert(i)
    }
  }

  // Expansion can be handled for rows as the universe is read in
  row += hasGalaxies ? 1 : expansion
}

// Columns have to be handled after the universe is injested.
for x in (0..<cols).reversed() {
  if colsWithGalaxies.contains(x) {
    continue
  }

  // Galaxies of a higher column need to have the expansion added.
  for i in 0..<universe.count {
    if universe[i].x > x {
      universe[i].x += expansion - 1
    }
  }
}

var total = 0

// Pairs of galaxies, exactly once. Reminds me of my C days.
for i in 0..<(universe.count - 1) {
  let galaxyA = universe[i]

  for j in (i + 1)..<universe.count {
    let galaxyB = universe[j]

    // Manhatten distance. Surprised it took until day 11 to see it!
    total += abs(galaxyA.y - galaxyB.y) + abs(galaxyA.x - galaxyB.x)
  }
}

print(total)
