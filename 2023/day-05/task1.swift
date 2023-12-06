var firstLine = readLine()!
var seeds = firstLine.split(separator: ": ")[1].split(separator: " ").compactMap { Int($0) }

var previousPlace: [Int: Int] = [:]
var currentPlace: [Int: Int] = seeds.reduce(into: [:], { x, y in x[y] = y })

while let line = readLine() {
  if line.isEmpty {
    continue
  }

  if line.first!.isLetter {
    previousPlace = currentPlace
  } else {
    let numbers = line.split(separator: " ").compactMap { Int($0) }
    let destinationStart = numbers[0]
    let sourceStart = numbers[1]
    let sourceEnd = numbers[1] + numbers[2]

    for (seed, previous) in previousPlace {
      if previous >= sourceStart && previous < sourceEnd {
        let offset = previous - sourceStart

        currentPlace[seed] = destinationStart + offset
      }
    }
  }
}

print(currentPlace.values.min()!)
