var total = 0

while let line = readLine() {
  let numerals = line.filter(\.isNumber)
  let first = numerals.first
  let last = numerals.last

  if let number = Int("\(first!)\(last!)") {
    total += number
  }
}

print(total)
