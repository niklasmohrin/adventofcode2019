from math import gcd
from itertools import combinations


class Moon:
    def __init__(self, pos):
        self.pos = pos
        self.vel = [0, 0, 0]

    @property
    def potential_energy(self):
        return sum(map(abs, self.pos))

    @property
    def kinetic_energy(self):
        return sum(map(abs, self.vel))

    @property
    def total_energy(self):
        return self.potential_energy * self.kinetic_energy


def apply_gravity(moons):
    for (m1, m2) in combinations(moons, 2):
        for dim in range(3):
            if m1.pos[dim] > m2.pos[dim]:
                m1.vel[dim] -= 1
                m2.vel[dim] += 1
            elif m1.pos[dim] < m2.pos[dim]:
                m1.vel[dim] += 1
                m2.vel[dim] -= 1


def apply_velocity(moons):
    for moon in moons:
        for dim in range(3):
            moon.pos[dim] += moon.vel[dim]


def total_energy(moons):
    return sum(map(lambda moon: moon.total_energy, moons))


def part_one():
    data = open("12/input.txt").read().strip()

    moons = [line.strip("<>").split(",") for line in data.split("\n")]
    moons = [Moon(list(map(lambda s: int(s.split("=")[1]), pos))) for pos in moons]

    for _ in range(1000):
        apply_gravity(moons)
        apply_velocity(moons)

    print(total_energy(moons))


def simulate1d(values):
    # apply gravity
    for (v1, v2) in combinations(values, 2):
        if v1[0] > v2[0]:
            v1[1] -= 1
            v2[1] += 1
        elif v1[0] < v2[0]:
            v1[1] += 1
            v2[1] -= 1
    # apply velocity
    for v in values:
        v[0] += v[1]
    return values


def turns_to_initial(initial):
    cur = [x.copy() for x in initial]
    cur = simulate1d(cur)
    steps = 1
    while any(cur[i] != initial[i] for i in range(len(initial))):
        steps += 1
        cur = simulate1d(cur)
    return steps


def lcm(a):
    lcm = a[0]
    for i in a[1:]:
        lcm = lcm * i // gcd(lcm, i)
    return lcm


def part_two():
    data = open("12/input.txt").read().strip()

    moons = [line.strip("<>").split(",") for line in data.split("\n")]
    moons = [Moon(list(map(lambda s: int(s.split("=")[1]), pos))) for pos in moons]

    x_initial = list(map(lambda moon: [moon.pos[0], moon.vel[0]], moons))
    y_initial = list(map(lambda moon: [moon.pos[1], moon.vel[1]], moons))
    z_initial = list(map(lambda moon: [moon.pos[2], moon.vel[2]], moons))

    x_turns = turns_to_initial(x_initial)
    y_turns = turns_to_initial(y_initial)
    z_turns = turns_to_initial(z_initial)

    print(lcm([x_turns, y_turns, z_turns]))


part_two()
