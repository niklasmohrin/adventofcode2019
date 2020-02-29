wire1, wire2 = open("03/input.txt").readlines()
# wire1 = "R8,U5,L5,D3"
# wire2 = "U7,R6,D4,L4"

# wire1, wire2 = """R75,D30,R83,U83,L12,D49,R71,U7,L72
# U62,R66,U55,R34,D71,R55,D58,R83""".split("\n")

# wire1, wire2 = """R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
# U98,R91,D20,R16,D67,R40,U7,R15,U6,R7""".splitlines()


def parse_input(wires):
    wires = wires.split(",")
    moves = []
    for wire in wires:
        direction, amount = wire[0], int(wire[1:])
        if direction in ['U', 'D']:
            moves.append((0, amount if direction == 'U' else -amount))
        elif direction in ['L', 'R']:
            moves.append((amount if direction == 'R' else -amount, 0))
        else:
            raise
    return moves


wire1 = parse_input(wire1)
wire2 = parse_input(wire2)


def fields(wire):
    s = set()
    dists = dict()
    x, y = 0, 0
    moves = 0
    for move in wire:
        # one of these is 0 => empty loop
        for dx in range(abs(move[0])):
            moves += 1
            x += 1 if move[0] > 0 else -1
            s.add((x, y))
            if (x, y) not in dists:
                dists[(x, y)] = moves
        for dy in range(abs(move[1])):
            moves += 1
            y += 1 if move[1] > 0 else -1
            s.add((x, y))
            if (x, y) not in dists:
                dists[(x, y)] = moves
    return s, dists


w1set, w1dists = fields(wire1)
w2set, w2dists = fields(wire2)


crossings = w1set.intersection(w2set)
crossings.discard((0, 0))


def dist(p):
    x, y = p
    return abs(x) + abs(y)


def actual_dist(p):
    return w1dists[p] + w2dists[p]


print(min(map(actual_dist, crossings)))
