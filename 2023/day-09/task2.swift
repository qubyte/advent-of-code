var total = 0

while let line = readLine() {
  var sequence = line.split(separator: " ").compactMap( { Int($0) })
  var differences = [sequence[0]]

  while Set(sequence).count != 1 {
    var nextSequence: [Int] = []

    for i in 1..<sequence.count {
      nextSequence.append(sequence[i] - sequence[i - 1])
    }

    differences.append(nextSequence[0])

    sequence = nextSequence
  }

  total += differences.reversed().reduce(0) { a, b in b - a }
}

print(total)
