var arrangements = 0

struct CacheKey: Hashable {
  let layout: [Character]
  let counts: [Int]
}

var cache: [CacheKey: Int] = [:]

func countArrangements(layout: [Character], counts: [Int]) -> Int {
  if layout.isEmpty {
    // No more layout. If no counts remain either then it's a match.
    return counts.count == 0 ? 1 : 0
  }
  if counts.count == 0 {
    // No more counts. If there are no '#' left in the layout then it's a match.
    return layout.contains("#") ? 0 : 1
  }

  let cacheKey = CacheKey(layout: layout, counts: counts)

  if let cached = cache[cacheKey] {
    return cached
  }

  var total = 0
  let firstChar = layout[0]

  if firstChar == "." || firstChar == "?" {
    // Slurp dots (move forward one character).
    total += countArrangements(layout: Array(layout[1...]), counts: counts);
  }

  if firstChar == "#" || firstChar == "?" {
    let firstCount = counts[0]

    // The first count can be satisfied when:
    // - the number of # expected is less than the remaining layout characters.
    // - the block contains no dots.
    // - the character after the block is not a hash (so the first count is not exceeded).
    if firstCount < layout.count && !layout[..<firstCount].contains(".") && layout[firstCount] != "#" {
      // A count can match, so move onto the next. Skip the leading dot.
      total += countArrangements(layout: Array(layout[(firstCount + 1)...]), counts: Array(counts[1...]))
    }
  }

  cache[cacheKey] = total

  return total
}

while let line = readLine() {
  let split = line.split(separator: " ").map { String($0) }
  let layout = Array(repeating: split[0], count: 5).joined(separator: "?") + "."
  let readCounts = split[1].split(separator: ",").compactMap { Int($0) }
  let counts = (0..<5).flatMap { _ in readCounts }

  arrangements += countArrangements(layout: Array(layout), counts: counts)
}

print(arrangements)
