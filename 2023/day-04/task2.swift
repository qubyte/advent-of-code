import Foundation

let numberRegex = try! Regex("\\d+")

var tickets: [Int: Int] = [:]
var ticketQueue: [Int] = []

while let line = readLine() {
  let winnersNumbersSplit = line.split(separator: " | ").map(String.init)
  let winnersChunk: String = winnersNumbersSplit[0]
  let numbersChunk: String = winnersNumbersSplit[1]
  let ticketNumberAndWinners = winnersChunk
    .matches(of: numberRegex)
    .compactMap { Int(winnersChunk[$0.range.lowerBound..<$0.range.upperBound]) }
  let ticketNumber = ticketNumberAndWinners[0]
  let winners = Set(ticketNumberAndWinners[1...])
  let numbers = Set(
    numbersChunk
      .matches(of: numberRegex)
      .compactMap { Int(numbersChunk[$0.range.lowerBound..<$0.range.upperBound]) }
  )
  let overlap = winners.intersection(numbers).count

  tickets[ticketNumber] = overlap
  ticketQueue.append(ticketNumber)
}

var count = 0

while let ticketIndex = ticketQueue.popLast() {
  count += 1

  let score = tickets[ticketIndex]!

  if score > 0 {
    ticketQueue.append(contentsOf: (ticketIndex + 1)...(ticketIndex + score))
  }
}

print(count)
