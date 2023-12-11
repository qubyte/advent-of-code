struct Coord: Hashable {
  let x: Int
  let y: Int
}

var universe: [Coord] = []
var row = 0
var cols = 0

while let line = readLine() {
  var count = 0

  cols = line.count

  for (i, char) in line.enumerated() {
    if char == "#" {
      count += 1
      universe.append(Coord(x: i, y: row))
    }
  }

  row += count == 0 ? 2 : 1
}

for x in (0..<cols).reversed() {
  let galaxiesInColumn = universe.filter { $0.x == x }.count
  var toRemove: [Coord] = []
  var toAdd: [Coord] = []

  if galaxiesInColumn == 0 {
    for galaxy in universe {
      if (galaxy.x > x) {
        toAdd.append(Coord(x: galaxy.x + 1, y: galaxy.y))
        toRemove.append(galaxy)
      }
    }
  }

  universe = universe.filter { !toRemove.contains($0) }
  universe.append(contentsOf: toAdd)
}

var total = 0

for i in 0..<(universe.count - 1) {
  let galaxyA = universe[i]

  for j in (i + 1)..<universe.count {
    let galaxyB = universe[j]

    total += abs(galaxyA.y - galaxyB.y) + abs(galaxyA.x - galaxyB.x)
  }
}

print(total)
