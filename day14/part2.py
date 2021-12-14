#!/usr/bin/env python3

s = """NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"""

s = open("input.txt").read()

template, rules = s.split("\n\n")
rules = list(map(lambda line: line.split(" -> "), rules.splitlines()))

pairs = {}
for i in range(len(template) - 1):
    pairs[template[i:(i+2)]] = 1

for i in range(40):
    changes = {}
    for (rule, insertee) in rules:
        if pairs.get(rule, 0) > 0:
            changes[rule] = changes.get(rule, 0) - pairs[rule]
            np1 = rule[0] + insertee
            np2 = insertee + rule[1]
            changes[np1] = changes.get(np1, 0) + pairs[rule]
            changes[np2] = changes.get(np2, 0) + pairs[rule]

    for pair, delta in changes.items():
        print(pair, delta)
        pairs[pair] = pairs.get(pair, 0) + delta

    print(pairs)

cleaned_pairs = {pair: count for pair, count in pairs.items() if count > 0}

counts = {}
for pair, count in cleaned_pairs.items():
    for i in (0, 1):
        counts[pair[i]] = counts.get(pair[i], 0) + count

counts = {c: (count + 1) // 2 for c, count in counts.items()}

diff = counts[max(counts, key=counts.get)] - counts[min(counts, key=counts.get)]
print(diff)
