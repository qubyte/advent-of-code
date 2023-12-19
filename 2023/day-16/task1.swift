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

var rays: [Ray] = [Ray(direction: .right, row: 0, col: 0)]
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

print(visited.count)
