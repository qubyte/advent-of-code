var firstLine = readLine()!
var seeds = firstLine.split(separator: ": ")[1].split(separator: " ").compactMap { Int($0) }
var seedRanges: [Range<Int>] = stride(from: 0, to: seeds.count, by: 2).map { seeds[$0]..<(seeds[$0] + seeds[$0 + 1]) }

var lowestLocation = Int.max
var transforms: [[(Int, Int, Int)]] = []

while let line = readLine() {
  if line.isEmpty {
    continue
  }

  if line.first!.isLetter {
    transforms.append([])
  } else {
    let numbers = line.split(separator: " ").compactMap { Int($0) }
    let sourceStart = numbers[1]
    let sourceEnd = numbers[1] + numbers[2]
    let destinationStart = numbers[0]

    transforms[transforms.count - 1].append((sourceStart, sourceEnd, destinationStart))
  }
}

for seedRange in seedRanges {
  for position in seedRange {
    var place = position

    outer: for stage in transforms {
      for (sourceStart, sourceEnd, destinationStart) in stage {
        if place >= sourceStart && place < sourceEnd {
          place += destinationStart - sourceStart
          continue outer
        }
      }
    }

    lowestLocation = min(lowestLocation, place)
  }
}

print(lowestLocation)
