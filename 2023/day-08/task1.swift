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

let directions = readLine()!.split(separator: "").map { $0 == "L" ? Direction.left : Direction.right }

while let line = readLine() {
  if line.isEmpty {
    continue
  }

  let labels = line.split(separator: notLabelCharacter).map { String($0) }
  let node = Node(label: labels[0], left: labels[1], right: labels[2])

  nodes[node.label] = node
}

var location: Node = nodes["AAA"]!
var step = 0

while location != nodes["ZZZ"]! {
  location = location.next(direction: directions[step % directions.count])
  step += 1
}

print(step)
