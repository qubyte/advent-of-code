import Foundation
import Algorithms

var arrangements = 0

while let line = readLine() {
  let split = line.split(separator: " ").map { String($0) }
  let layout = String(split[0])
  let counts = split[1].split(separator: ",").compactMap { Int($0) }

  let brokenSum = counts.reduce(0, +)
  let brokenAccounted = layout.reduce(0) { $0 + ($1 == "#" ? 1 : 0) }
  let brokenUnaccounted = brokenSum - brokenAccounted
  let unknownIndices = layout.enumerated().compactMap { (i, c) in c == "?" ? i : nil }

  var arrangementsForThisLine = 0

  // brokenAccounted in unknownIndices
  for combo in unknownIndices.combinations(ofCount: brokenUnaccounted) {
    var characters = Array(layout)

    for (i, c) in layout.enumerated() {
      if c == "?" {
        characters[i] = combo.contains(i) ? "#" : "."
      }
    }

    let joined: String = characters.map { String($0) }.joined()
    let pattern = joined.split(separator: ".").map { String($0) }.map { $0.count }

    if pattern == counts {
      arrangementsForThisLine += 1
    }
  }

  arrangements += arrangementsForThisLine
}

print(arrangements)
