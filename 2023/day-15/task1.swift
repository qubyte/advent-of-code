var current = 0
var sum = 0
let line = readLine()!

for character in line {
  if character == "," {
    sum += current
    current = 0
  } else if let asciiVal = character.asciiValue {
    current += Int(asciiVal)
    current *= 17
    current %= 256
  }
}

sum += current

print(sum)
