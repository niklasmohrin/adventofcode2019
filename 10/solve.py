from math import gcd, atan2, sqrt, pi
from collections import defaultdict
from queue import PriorityQueue
from itertools import cycle

data = open("10/input.txt").read().strip().split("\n")

# data = """......#.#.
# #..#.#....
# ..#######.
# .#.#.###..
# .#..#.....
# ..#....#.#
# #..#....#.
# .##.#..###
# ##...#..#.
# .#....####""".split("\n")

# data = """#.#...#.#.
# .###....#.
# .#....#...
# ##.#.#.#.#
# ....#.#.#.
# .##..###.#
# ..#...##..
# ..##....##
# ......#...
# .####.###.""".split("\n")

# data = """.#..#..###
# ####.###.#
# ....###.#.
# ..###.##.#
# ##.##.#.#.
# ....###..#
# ..#.#..#.#
# #..#.#.###
# .##...##.#
# .....#.#..""".split("\n")

# data = """.#..##.###...#######
# ##.############..##.
# .#.######.########.#
# .###.#######.####.#.
# #####.##.#.##.###.##
# ..#####..#.#########
# ####################
# #.####....###.#.#.##
# ##.#################
# #####.##.###..####..
# ..######..##.#######
# ####.##.####...##..#
# .#####..#.######.###
# ##...#.##########...
# #.##########.#######
# .####.#.###.###.#.##
# ....##.##.###..#####
# .#.#.###########.###
# #.#.#.#####.####.###
# ###.##.####.##.#..##""".split("\n")

ASTEROID = '#'
NOTHING = '.'

height = len(data)
width = len(data[0])

asteroids = [(x, y) for y, row in enumerate(data) for x, field in enumerate(row) if field == ASTEROID]

direct_line_of_sight = set()

for i, (x1, y1) in enumerate(asteroids):
    for (x2, y2) in asteroids[i + 1:]:
        dx = x2 - x1
        dy = y2 - y1
        x = gcd(dx, dy)
        dx /= x
        dy /= x
        for j in range(1, x):
            if (x1 + j * dx, y1 + j * dy) in asteroids:
                direct_line_of_sight.add(((x1, y1), (x1 + j * dx, y1 + j * dy)))
                break
        else:
            direct_line_of_sight.add(((x1, y1), (x2, y2)))


def count_visible(ast):
    return len(list(filter(lambda pair: ast in pair, direct_line_of_sight)))


station = max(asteroids, key=count_visible)
print(f"Station is at {station} with {count_visible(station)} asteroids visible.")


def angle_from_station(ast):
    x, y = ast
    x -= station[0]
    y -= station[1]
    return (atan2(y, x) - pi * 3 / 2) % (2 * pi)


def radius_from_station(ast):
    x, y = ast
    x -= station[0]
    y -= station[1]
    return sqrt(x * x + y * y)


asteroids_by_angle = defaultdict(PriorityQueue)
for ast in asteroids:
    angle = angle_from_station(ast)
    radius = radius_from_station(ast)
    asteroids_by_angle[angle].put((radius, ast))

i = 1

for angle in cycle(sorted(asteroids_by_angle.keys())):
    if asteroids_by_angle[angle].empty():
        continue

    ast = asteroids_by_angle[angle].get()[1]
    # print(i, ast)
    if i == 200:
        print(f"The 200th asteroid hit is at {ast}.")
        print(f"The solution is {ast[0] * 100 + ast[1]}.")
        break
    else:
        i += 1
