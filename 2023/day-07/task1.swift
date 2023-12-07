typealias Cards = (Int, Int, Int, Int, Int)

func rawCardToInt(rawCard: String) -> Int? {
  switch rawCard {
  case "2": return 0
  case "3": return 1
  case "4": return 2
  case "5": return 3
  case "6": return 4
  case "7": return 5
  case "8": return 6
  case "9": return 7
  case "T": return 8
  case "J": return 9
  case "Q": return 10
  case "K": return 11
  case "A": return 12
  default: return nil
  }
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

  let counts: [Int: Int] = rawCards.reduce(into: [:], { hash, card in
    hash[card] = (hash[card] ?? 0) + 1
  })

  var handKind: HandKind

  switch counts.count {
  case 1: handKind = HandKind.fiveOfAKind
  case 2: handKind = counts.values.max() == 3 ? HandKind.fullHouse : HandKind.fourOfAKind
  case 3: handKind = counts.values.max() == 2 ? HandKind.twoPair : HandKind.threeOfAKind
  case 4: handKind = HandKind.onePair
  default: handKind = HandKind.highCard
  }

  hands.append(Hand(cards: cards, kind: handKind, bid: bid))
}

hands.sort()

var total = 0

for (index, hand) in hands.enumerated() {
  total += hand.bid * (index + 1)
}

print(total)
