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
  ticketQueue.insert(ticketNumber, at: 0)
}

var index = 0

while index < ticketQueue.count {
  let ticketIndex = ticketQueue[index]
  let score = tickets[ticketIndex]!

  if score > 0 {
    for ticketNumber in (ticketIndex + 1)...(ticketIndex + score) {
      ticketQueue.append(ticketNumber)
    }
  }

  index += 1
}

print(ticketQueue.count)
