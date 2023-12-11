struct Coord: Hashable {
  let row: Int
  let col: Int
}

var lengths: [Coord: [Coord]] = [:]
var row = 0
var start = Coord(row: -1, col: -1)

while let line = readLine() {
  let maxCol = line.count - 1

  for (col, char) in line.enumerated() {
    let loc = Coord(row: row, col: col)

    var ends: [Coord] = []

    switch char {
    case "S":
      start = Coord(row: row, col: col)
      continue;
    case "|":
      if row > 0 {
        ends.append(Coord(row: row - 1, col: col))
      }
      ends.append(Coord(row: row + 1, col: col)) // TODO: Last row may need pruning
    case "-":
      if col > 0 {
        ends.append(Coord(row: row, col: col - 1))
      }
      if col < maxCol {
        ends.append(Coord(row: row, col: col + 1))
      }
    case "L":
      if row > 0 {
        ends.append(Coord(row: row - 1, col: col))
      }
      if col < maxCol {
        ends.append(Coord(row: row, col: col + 1))
      }
    case "J":
      if row > 0 {
        ends.append(Coord(row: row - 1, col: col))
      }
      if col > 0 {
        ends.append(Coord(row: row, col: col - 1))
      }
    case "7":
      if col > 0 {
        ends.append(Coord(row: row, col: col - 1))
      }
      ends.append(Coord(row: row + 1, col: col)) // TODO: Last row may need pruning
    case "F":
      if col < maxCol {
        ends.append(Coord(row: row, col: col + 1))
      }
      ends.append(Coord(row: row + 1, col: col)) // TODO: Last row may need pruning
    default:
      continue;
    }

    if ends.count > 0 {
      lengths[loc] = ends
    }
  }

  row += 1
}

// Prune pipes with only one connection.
for (loc, connections) in lengths {
  let connection_count = connections.filter { coord in coord.row != row + 1 }.count

  if connection_count != 2 {
    lengths.removeValue(forKey: loc)
  }
}

var connectionsToStart: [Coord] = []

// Find a connection to start
for (loc, connections) in lengths {
  if connections.contains(start) {
    connectionsToStart.append(loc)
  }
}

var thisLoc = connectionsToStart[0]
var visited = Set([thisLoc])

while !visited.contains(start) {
  let connections = lengths[thisLoc]!

  for loc in connections {
    if !visited.contains(loc) {
      visited.insert(loc)
      thisLoc = loc
      break
    }
  }
}

print(visited.count / 2)
