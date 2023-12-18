var sum = 0
let line = readLine()!
var register: [[(String, Int)]] = Array(repeating: [], count: 256)

func calculateHash(_ instruction: String) -> Int {
  var current = 0

  for character in instruction {
    if let asciiVal = character.asciiValue {
      current += Int(asciiVal)
      current *= 17
      current %= 256
    }
  }

  return current
}

for instruction in line.split(separator: ",") {
  if instruction.last == "-" {
    let label = String(instruction.dropLast())
    let hash = calculateHash(label)

    if let index = register[hash].firstIndex(where: { $0.0 == label }) {
      register[hash].remove(at: index)
    }
  } else {
    let parts = instruction.split(separator: "=").map { String($0) }
    let label = parts[0]
    let hash = calculateHash(label)
    let focalLength = Int(parts[1])!

    if let index = register[hash].firstIndex(where: { $0.0 == label }) {
      register[hash][index] = (label, focalLength)
    } else {
      register[hash].append((label, focalLength))
    }
  }
}

for (i, box) in register.enumerated() {
  for (j, (_, focalLength)) in box.enumerated() {
    sum += (i + 1) * (j + 1) * focalLength
  }
}

print(sum)
