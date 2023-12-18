var platform: [[Character]] = []

while let line = readLine() {
  platform.append(Array(line))

  let row = platform.count - 1

  for i in 0..<platform[row].count {
    for j in (0..<row).reversed() {
      if platform[j + 1][i] == "O" && platform[j][i] == "." {
        platform[j + 1][i] = "."
        platform[j][i] = "O"
      } else if platform[j + 1][i] == "#" {
        continue
      }
    }
  }
}

var total = 0

for (j, row) in platform.enumerated() {
  for char in row {
    if char == "O" {
      total += platform.count - j
    }
  }
}

print(total)
