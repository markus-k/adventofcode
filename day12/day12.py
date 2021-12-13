#!/usr/bin/env python3

example1 = """start-A
start-b
A-c
A-b
b-d
A-end
b-end"""

example2 = """dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"""

example3 = """fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"""

puzzle_input = open("input.txt", "r").read()


def get_adjacents(edges: list) -> dict:
    adjacents = {}
    for n1, n2 in edges:
        if n1 not in adjacents:
            adjacents[n1] = []
        if n2 not in adjacents:
            adjacents[n2] = []
        adjacents[n1].append(n2)
        adjacents[n2].append(n1)

    return adjacents


def explore(adjacents, start, explored):
    explored.append(start)

    tree = {}
    for node in adjacents[start]:
        if node == "end":
            tree[node] = {}
        elif node != "start" and not (node.islower() and node in explored):
            tree[node] = explore(adjacents, node, explored.copy())

    return tree


def count_ends(tree):
    count = 0
    for node in tree:
        if node == "end":
            count += 1
        count += count_ends(tree[node])
    return count


def part1(s: str) -> int:
    edges = list(map(lambda x: x.split("-"), s.splitlines()))
    adjacents = get_adjacents(edges)

    explored = []
    tree = explore(adjacents, "start", explored)
    count = count_ends(tree)

    return count


if __name__ == "__main__":
    assert part1(example1) == 10
    assert part1(example2) == 19
    assert part1(example3) == 226

    print(f"Paths in cave: {part1(puzzle_input)}")
