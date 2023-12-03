var grid: [[Character]] = []

while let line = readLine() {
  grid.append(contentsOf: [Array(line)])
}

let height = grid.count
let width = grid[0].count

var total = 0
var adjacentSymbol = false
var batch = ""

func checkNeighboursForSymbol(j: Int, i: Int) -> Bool {
  for y in max(0, j - 1)...min(height - 1, j + 1) {
    for x in max(0, i - 1)...min(width - 1, i + 1) {
      let char = grid[y][x]

      if !char.isWholeNumber && char != "." {
        return true;
      }
    }
  }

  return false
}

func finalizeGroup() {
  if adjacentSymbol {
    if let number = Int(batch) {
      total += number
    }
    adjacentSymbol = false
  }
  batch = ""
}

for y in 0..<height {
  for x in 0..<width {
    let char = grid[y][x]

    if char.isWholeNumber {
      batch += String(char)
      adjacentSymbol = adjacentSymbol || checkNeighboursForSymbol(j: y, i: x)
    } else {
      finalizeGroup()
    }
  }

  finalizeGroup()
}

print(total)
