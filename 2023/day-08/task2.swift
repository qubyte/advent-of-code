enum Direction {
  case left, right
}

let notLabelCharacter = try! Regex("[^A-Z]")

var nodes: [String: Node] = [:]

struct Node: Equatable {
  var label: String
  var left: String
  var right: String

  func next(direction: Direction) -> Node {
    switch direction {
    case Direction.left: return nodes[left]!
    case Direction.right: return nodes[right]!
    }
  }
}

func gcd(x: Int, y: Int) -> Int {
  var a = 0
  var b = max(x, y)
  var r = min(x, y)

  while r != 0 {
    a = b
    b = r
    r = a % b
  }

  return b
}

func lcm(x: Int, y: Int) -> Int {
  return x / gcd(x: x, y: y) * y
}

let directions = readLine()!.split(separator: "").map { $0 == "L" ? Direction.left : Direction.right }

var starts: [Node] = []

while let line = readLine() {
  if line.isEmpty {
    continue
  }

  let labels = line.split(separator: notLabelCharacter).map { String($0) }
  let node = Node(label: labels[0], left: labels[1], right: labels[2])

  nodes[node.label] = node

  if labels[0].hasSuffix("A") {
    starts.append(node)
  }
}

var answer = -1

for start in starts {
  var location = start
  var steps = 0

  while !location.label.hasSuffix("Z") {
    location = location.next(direction: directions[steps % directions.count])
    steps += 1
  }

  if answer < 0 {
    answer = steps
  } else {
    answer = lcm(x: answer, y: steps)
  }
}

print(answer)
