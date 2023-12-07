typealias Cards = (Int, Int, Int, Int, Int)

let cards = ["J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A"]

func rawCardToInt(rawCard: String) -> Int? {
  cards.firstIndex(of: rawCard)
}

enum HandKind: Comparable {
  case highCard, onePair, twoPair, threeOfAKind, fullHouse, fourOfAKind, fiveOfAKind
}

struct Hand: Comparable {
  let cards: Cards
  let kind: HandKind
  let bid: Int

  static func < (lhs: Hand, rhs: Hand) -> Bool {
    if lhs.kind < rhs.kind {
      return true
    }
    if lhs.kind > rhs.kind {
      return false
    }
    return lhs.cards < rhs.cards
  }

  static func == (lhs: Hand, rhs: Hand) -> Bool {
    return lhs.cards == rhs.cards
  }
}

var hands: [Hand] = []

while let line = readLine() {
  let split = line.split(separator: " ").map { String($0) }
  let rawCards = split[0].split(separator: "").compactMap { rawCardToInt(rawCard: String($0)) }
  let cards = (rawCards[0], rawCards[1], rawCards[2], rawCards[3], rawCards[4])
  let bid = Int(split[1])!

  var jackCount = 0
  var counts: [Int: Int] = [:]

  for card in rawCards {
    if card == 0 {
      jackCount += 1
    } else {
      counts[card] = (counts[card] ?? 0) + 1
    }
  }

  var handKind: HandKind

  switch jackCount {
  case 5, 4: handKind = HandKind.fiveOfAKind
  case 3: handKind = counts.count == 1 ? HandKind.fiveOfAKind : HandKind.fourOfAKind
  case 2:
    switch counts.count {
    case 3: handKind = HandKind.threeOfAKind
    case 2: handKind = HandKind.fourOfAKind
    default: handKind = HandKind.fiveOfAKind
    }
  case 1:
    switch counts.count {
    case 4: handKind = HandKind.onePair
    case 3: handKind = HandKind.threeOfAKind
    case 2: handKind = counts.values.max() == 2 ? HandKind.fullHouse : HandKind.fourOfAKind
    default: handKind = HandKind.fiveOfAKind
    }
  default:
    switch counts.count {
    case 1: handKind = HandKind.fiveOfAKind
    case 2: handKind = counts.values.max() == 3 ? HandKind.fullHouse : HandKind.fourOfAKind
    case 3: handKind = counts.values.max() == 2 ? HandKind.twoPair : HandKind.threeOfAKind
    case 4: handKind = HandKind.onePair
    default: handKind = HandKind.highCard
    }
  }

  hands.append(Hand(cards: cards, kind: handKind, bid: bid))
}

hands.sort()

var total = 0

for (index, hand) in hands.enumerated() {
  total += hand.bid * (index + 1)
}

print(total)
