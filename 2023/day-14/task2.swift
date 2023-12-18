var platform: [[Character]] = []
var previousSteps: [[[Character]]] = []

func rotateNorth() {
  for i in 0..<platform[0].count {
    let col = (0..<platform.count).map { platform[$0][i] }
    let partitioned = col.split(separator: "#", omittingEmptySubsequences: false).compactMap { Array($0) }.map { $0.sorted { $1 < $0 } }.joined(separator: ["#"])

    for (j, char) in partitioned.enumerated() {
      platform[j][i] = char
    }
  }
}

func rotateEast() {
  for (j, row) in platform.enumerated() {
    let partitioned = row.split(separator: "#", omittingEmptySubsequences: false).compactMap { Array($0) }.map { $0.sorted { $0 < $1 } }.joined(separator: ["#"])

    for (i, char) in partitioned.enumerated() {
      platform[j][i] = char
    }
  }
}

func rotateSouth() {
  for i in 0..<platform[0].count {
    let col = (0..<platform.count).map { platform[$0][i] }
    let partitioned = col.split(separator: "#", omittingEmptySubsequences: false).compactMap { Array($0) }.map { $0.sorted { $0 < $1 } }.joined(separator: ["#"])

    for (j, char) in partitioned.enumerated() {
      platform[j][i] = char
    }
  }
}

func rotateWest() {
  for (j, row) in platform.enumerated() {
    let partitioned = row.split(separator: "#", omittingEmptySubsequences: false).compactMap { Array($0) }.map { $0.sorted { $1 < $0 } }.joined(separator: ["#"])

    for (i, char) in partitioned.enumerated() {
      platform[j][i] = char
    }
  }
}

func cycle() {
  rotateNorth()
  rotateWest()
  rotateSouth()
  rotateEast()
}

while let line = readLine() {
  platform.append(Array(line))
}

previousSteps.append(platform)

var firstIndex = -1

while true {
  cycle()

  previousSteps.append(platform)

  if let index = previousSteps.firstIndex(of: platform) {
    if index < (previousSteps.count - 1) {
      firstIndex = index
      break
    }
  }
}

let lastIndex = previousSteps.count
let interval = lastIndex - firstIndex - 1

for _ in 0..<((1000000000 - lastIndex + 1) % interval) {
  cycle()
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
