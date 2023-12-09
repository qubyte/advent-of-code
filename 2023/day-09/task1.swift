var total = 0

while let line = readLine() {
  var sequence = line.split(separator: " ").compactMap( { Int($0) })
  var differences = [sequence.last!]

  while Set(sequence).count != 1 {
    var nextSequence: [Int] = []

    for i in 1..<sequence.count {
      nextSequence.append(sequence[i] - sequence[i - 1])
    }

    differences.append(nextSequence.last!)

    sequence = nextSequence
  }

  total += differences.reduce(0) { a, b in a + b }
}

print(total)
