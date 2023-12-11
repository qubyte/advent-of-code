struct Coord: Hashable {
  let row: Int
  let col: Int

  func neighbour(connection: Connection) -> Coord {
    switch connection {
    case Connection.up: return Coord(row: row - 1, col: col)
    case Connection.down: return Coord(row: row + 1, col: col)
    case Connection.left: return Coord(row: row, col: col - 1)
    case Connection.right: return Coord(row: row, col: col + 1)
    }
  }
}

enum Connection: CaseIterable {
  case up, down, left, right
}

var lengths: [Coord: [Connection]] = [:]
var row = 0
var start = Coord(row: -1, col: -1)
var maxCol = -1

while let line = readLine() {
  maxCol = line.count - 1

  for (col, char) in line.enumerated() {
    let loc = Coord(row: row, col: col)

    var ends: [Connection] = []

    switch char {
    case "S":
      start = Coord(row: row, col: col)
      continue;
    case "|":
      if row > 0 {
        ends.append(Connection.up)
      }
      ends.append(Connection.down)
    case "-":
      if col > 0 {
        ends.append(Connection.left)
      }
      if col < maxCol {
        ends.append(Connection.right)
      }
    case "L":
      if row > 0 {
        ends.append(Connection.up)
      }
      if col < maxCol {
        ends.append(Connection.right)
      }
    case "J":
      if row > 0 {
        ends.append(Connection.up)
      }
      if col > 0 {
        ends.append(Connection.left)
      }
    case "7":
      if col > 0 {
        ends.append(Connection.left)
      }
      ends.append(Connection.down)
    case "F":
      if col < maxCol {
        ends.append(Connection.right)
      }
      ends.append(Connection.down)
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
  let connection_count = connections.filter { loc.row != row || $0 != Connection.down }.count

  if connection_count != 2 {
    lengths.removeValue(forKey: loc)
  }
}

var connectionsToStart: [Coord] = []

for connection in Connection.allCases {
  let neighbour = start.neighbour(connection: connection)
  let connections = lengths[neighbour]

  let hasStart = connections?.contains { connection in
    neighbour.neighbour(connection: connection) == start
  }

  if hasStart == true {
    connectionsToStart.append(neighbour)
  }
}

var previous = start
var thisLoc = connectionsToStart[0]
var visited = Set([thisLoc])

while !visited.contains(start) {
  let connections = lengths[thisLoc]!

  for connection in connections {
    let next = thisLoc.neighbour(connection: connection)

    if next == previous {
      continue
    }

    if !visited.contains(next) {
      visited.insert(next)
      previous = thisLoc
      thisLoc = next
      break
    }
  }
}

var doubledGrid = Array(repeating: Array(repeating: 0, count: maxCol * 2 + 2), count: row * 2 + 2)

for loc in visited {
  let projectedRow = 1 + 2 * loc.row
  let projectedCol = 1 + 2 * loc.col

  doubledGrid[projectedRow][projectedCol] = 1

  if loc == start {
    continue
  }

  let connections = lengths[loc]!

  if connections.contains(Connection.right) {
    doubledGrid[projectedRow][projectedCol + 1] = 1
  }
  if connections.contains(Connection.down) {
    doubledGrid[projectedRow + 1][projectedCol] = 1
  }
  if connections.contains(Connection.left) {
    doubledGrid[projectedRow][projectedCol - 1] = 1
  }
  if connections.contains(Connection.up) {
    doubledGrid[projectedRow - 1][projectedCol] = 1
  }
}

var queue = Set([Coord(row: 0, col: 0)])

while let loc = queue.popFirst() {
  let row = loc.row
  let col = loc.col

  if doubledGrid[row][col] != 0 {
    continue
  }

  doubledGrid[row][col] = -1

  if row > 0 && doubledGrid[row - 1][col] == 0 {
    queue.insert(Coord(row: row - 1, col: col))
  }
  if row < doubledGrid.count - 1 && doubledGrid[row + 1][col] == 0 {
    queue.insert(Coord(row: row + 1, col: col))
  }
  if col > 0 && doubledGrid[row][col - 1] == 0 {
    queue.insert(Coord(row: row, col: col - 1))
  }
  if col < doubledGrid[0].count - 1 && doubledGrid[row][col + 1] == 0 {
    queue.insert(Coord(row: row, col: col + 1))
  }
}

var total = 0

for j in 0..<row {
  for i in 0..<maxCol {
    let row = 1 + j * 2
    let col = 1 + i * 2

    if doubledGrid[row][col] == 0 {
      total += 1
    }
  }
}

print(total)
