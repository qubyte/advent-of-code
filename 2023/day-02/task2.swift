var total = 0

while let line = readLine() {
  let splits = line.split(separator: " ")

  var maxRed = 0
  var maxGreen = 0
  var maxBlue = 0

  for i in 1..<splits.count {
    let chunk = splits[i]

    if (chunk.starts(with: "red")) {
      maxRed = max(Int(splits[i - 1])!, maxRed)
    } else if (chunk.starts(with: "green")) {
      maxGreen = max(Int(splits[i - 1])!, maxGreen)
    } else if (chunk.starts(with: "blue")) {
      maxBlue = max(Int(splits[i - 1])!, maxBlue)
    }
  }

  total += maxRed * maxGreen * maxBlue
}

print(total)
