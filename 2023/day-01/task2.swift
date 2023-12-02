import Foundation

var total = 0

let matchers: [String: Character] = [
  "0": "0",
  "1": "1",
  "2": "2",
  "3": "3",
  "4": "4",
  "5": "5",
  "6": "6",
  "7": "7",
  "8": "8",
  "9": "9",
  "one": "1",
  "two": "2",
  "three": "3",
  "four": "4",
  "five": "5",
  "six": "6",
  "seven": "7",
  "eight": "8",
  "nine": "9"
]

func getFirstIndex(string: String, substring: String) -> Int {
  if let range = string.range(of: substring) {
    string.distance(from: string.startIndex, to: range.lowerBound)
  } else {
    Int.max
  }
}

func getLastIndex(string: String, substring: String) -> Int {
  if let range = string.range(of: substring, options: .backwards) {
    string.distance(from: string.startIndex, to: range.lowerBound)
  } else {
    -1
  }
}

while let line = readLine() {
  var first: Character? = nil
  var firstIndex = Int.max
  var last: Character? = nil
  var lastIndex = -1

  for (k, v) in matchers {
    let firstIndexOfThisOne = getFirstIndex(string: line, substring: k)
    let lastIndexOfThisOne = getLastIndex(string: line, substring: k)

    if firstIndexOfThisOne < firstIndex {
      firstIndex = firstIndexOfThisOne
      first = v
    }
    if lastIndexOfThisOne > lastIndex {
      lastIndex = lastIndexOfThisOne
      last = v
    }
  }

  if let f = first, let l = last, let number = Int("\(f)\(l)") {
    total += number
  }
}

print(total)
