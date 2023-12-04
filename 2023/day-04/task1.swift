import Foundation

let numberRegex = try! Regex("\\d+")

var total = 0

while let line = readLine() {
  let winnersNumbersSplit = line.split(separator: ":")[1].split(separator: " | ").map(String.init)
  let winnersChunk: String = winnersNumbersSplit[0]
  let numbersChunk: String = winnersNumbersSplit[1]
  let winners = Set(
    winnersChunk
      .matches(of: numberRegex)
      .compactMap { Int(winnersChunk[$0.range.lowerBound..<$0.range.upperBound]) }
  )
  let numbers = Set(
    numbersChunk
      .matches(of: numberRegex)
      .compactMap { Int(numbersChunk[$0.range.lowerBound..<$0.range.upperBound]) }
  )
  let overlap = winners.intersection(numbers).count

  if overlap > 0 {
    total += (0..<(overlap - 1)).reduce(1, { x, _ in x * 2 })
  }
}

print(total)
