enum Direction {
  case left, right, up, down
}

var grid: [[Character]] = []

struct Coord: Hashable {
  let row: Int
  let col: Int
}

struct Ray: Hashable {
  let direction: Direction
  let row: Int
  let col: Int

  func advance() -> Ray? {
    switch direction {
    case Direction.left: return col == 0 ? nil : Ray(direction: direction, row: row, col: col - 1)
    case Direction.right: return col == (grid[row].count - 1) ? nil : Ray(direction: direction, row: row, col: col + 1)
    case Direction.up: return row == 0 ? nil : Ray(direction: direction, row: row - 1, col: col)
    case Direction.down: return row == (grid.count - 1) ? nil : Ray(direction: direction, row: row + 1, col: col)
    }
  }

  func orient() -> [Ray] {
    let cell = grid[row][col]

    switch cell {
    case ".": return [self]
    case "-":
      switch direction {
      case .up, .down: return [
        Ray(direction: .left, row: row, col: col),
        Ray(direction: .right, row: row, col: col)
      ]
      default: return [self]
      }
    case "|":
      switch direction {
      case .left, .right: return [
        Ray(direction: .up, row: row, col: col),
        Ray(direction: .down, row: row, col: col)
      ]
      default: return [self]
      }
    case "/":
      switch direction {
      case .up: return [Ray(direction: .right, row: row, col: col)]
      case .down: return [Ray(direction: .left, row: row, col: col)]
      case .left: return [Ray(direction: .down, row: row, col: col)]
      case .right: return [Ray(direction: .up, row: row, col: col)]
      }
    case "\\":
      switch direction {
      case .up: return [Ray(direction: .left, row: row, col: col)]
      case .down: return [Ray(direction: .right, row: row, col: col)]
      case .left: return [Ray(direction: .up, row: row, col: col)]
      case .right: return [Ray(direction: .down, row: row, col: col)]
      }
    default: return []
    }
  }
}

while let line = readLine() {
  grid.append(Array(line))
}

let maxRow = grid.count - 1
let maxCol = grid[0].count - 1

var starts: [Ray] = []

for j in 0..<grid.count {
  starts.append(Ray(direction: .right, row: j, col: 0))
  starts.append(Ray(direction: .left, row: j, col: maxCol))
}
for i in 0..<grid[0].count {
  starts.append(Ray(direction: .down, row: 0, col: i))
  starts.append(Ray(direction: .up, row: maxRow, col: i))
}

var most = 0

for start in starts {
  var rays: [Ray] = [start]
  var raysKnown: Set<Ray> = []

  while true {
    var orientedRays: [Ray] = []

    for ray in rays {
      let next = ray.orient()

      for ray in next {
        if raysKnown.insert(ray).inserted {
          orientedRays.append(ray)
        }
      }
    }

    if orientedRays.count == 0 {
      break
    }

    rays = orientedRays.compactMap { $0.advance() }
  }

  var visited: Set<Coord> = Set()

  for ray in raysKnown {
    visited.insert(Coord(row: ray.row, col: ray.col))
  }

  most = max(most, visited.count)
}


print(most)
