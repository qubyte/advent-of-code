struct PartNumber {
  var number: Int, y: Int, xStart: Int, length: Int

  func inNeighborhood(y: Int, x: Int) -> Bool {
    y >= self.y - 1 && y <= self.y + 1 && x >= xStart - 1 && x <= xStart + length
  }
}

var partNumbers: [PartNumber] = []
var gearCoords: [(y: Int, x: Int)] = []
var row = 0
var total = 0

while let line = readLine() {
  var partNumber = "", length = 0, xStart = -1

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

outer: for (y, x) in gearCoords {
  var ratio = 1
  var count = 0

  for partNumber in partNumbers {
    if partNumber.inNeighborhood(y: y, x: x) {
      if count == 2 {
        continue outer
      }
      ratio *= partNumber.number
      count += 1
    }
  }

  if count == 2 {
    total += ratio
  }
}

print(total)
