struct Pattern {
  let width: Int
  let height: Int
  let grid: [[Bool]]

  func transpose() -> Pattern {
    var transposedGrid: [[Bool]] = []

    for i in 0..<width {
      transposedGrid.append(grid.map { $0[i] })
    }

    return Pattern(width: height, height: width, grid: transposedGrid)
  }
}

var patterns: [Pattern] = []
var patternBuffer: [[Bool]] = []
var columnsToTheLeft = 0
var rowsAbove = 0

func findRowIndex(_ pattern: Pattern) -> Int? {
  outer: for j in 1..<pattern.height {
    var fixes = 0

    for gap in 0..<pattern.height {
      if j < 1 + gap || j + gap >= pattern.height {
        break
      }

      let rowA = pattern.grid[j - 1 - gap]
      let rowB = pattern.grid[j + gap]

      for i in 0..<rowA.count {
        if rowA[i] != rowB[i] {
          fixes += 1
        }
      }

      if fixes > 1 {
        continue outer
      }
    }

    if fixes == 1 {
      return j
    }
  }

  return nil
}

func finalizePattern() {
  let width = patternBuffer[0].count
  let height = patternBuffer.count
  let pattern = Pattern(width: width, height: height, grid: patternBuffer)

  patterns.append(pattern)
  patternBuffer = []

  if let row = findRowIndex(pattern) {
    rowsAbove += row
  } else if let col = findRowIndex(pattern.transpose()) {
    columnsToTheLeft += col
  }
}

while let line = readLine() {
  if line.isEmpty {
    finalizePattern()
  } else {
    patternBuffer.append(line.split(separator: "").map { $0 == "#" })
  }
}

finalizePattern()

print(columnsToTheLeft + 100 * rowsAbove)
