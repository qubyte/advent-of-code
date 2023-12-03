import Foundation

struct PartNumber {
  var number: Int
  var y: Int
  var xStart: Int
  var length: Int

  func inNeighborhood(y: Int, x: Int) -> Bool {
    y >= self.y - 1 && y <= self.y + 1 && x >= xStart - 1 && x <= xStart + length
  }
}

var partNumbers: [PartNumber] = []
var gearCoords: [(y: Int, x: Int)] = []
var row = 0;
var total = 0

while let line = readLine() {
  var partNumber = ""
  var length = 0
  var xStart = -1

  for (x, char) in line.enumerated() {
    if char.isWholeNumber {
      if xStart == -1 {
        xStart = x
      }
      partNumber += String(char)
      length += 1
    } else {
      if let number = Int(partNumber) {
        partNumbers.append(PartNumber(number: number, y: row, xStart: xStart, length: length))
      }

      partNumber = ""
      length = 0
      xStart = -1

      if char == "*" {
        gearCoords.append((row, x))
      }
    }
  }

  if let number = Int(partNumber) {
    partNumbers.append(PartNumber(number: number, y: row, xStart: xStart, length: length))
  }

  row += 1
}

for (y, x) in gearCoords {
  let matches = partNumbers
    .filter { p in p.inNeighborhood(y: y, x: x) }
    .map { p in p.number }

  if matches.count == 2 {
    total += matches[0] * matches[1]
  }
}

print(total)
