import Foundation

let MAX_RED = 12
let MAX_GREEN = 13
let MAX_BLUE = 14

var total = 0

while let line = readLine() {
  let gameNumber = Int(line.split(separator: ":", maxSplits: 1)[0].filter(\.isNumber))!
  let splits = line.split(separator: " ")

  var maxRed = 0
  var maxGreen = 0
  var maxBlue = 0

  for i in 1...splits.count - 1 {
    let chunk = splits[i]

    if (chunk.starts(with: "red")) {
      maxRed = max(Int(splits[i - 1])!, maxRed)
    } else if (chunk.starts(with: "green")) {
      maxGreen = max(Int(splits[i - 1])!, maxGreen)
    } else if (chunk.starts(with: "blue")) {
      maxBlue = max(Int(splits[i - 1])!, maxBlue)
    }
  }

  if maxRed <= MAX_RED && maxGreen <= MAX_GREEN && maxBlue <= MAX_BLUE {
    total += gameNumber
  }
}

print(total)
