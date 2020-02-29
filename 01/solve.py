masses = map(int, open("01/input.txt").readlines())
# masses = [100756]


def fuel_needed(mass):
    return mass // 3 - 2


def fuel_really_needed(mass):
    fuel = 0
    next_step = fuel_needed(mass)
    while next_step > 0:
        fuel += next_step
        next_step = fuel_needed(next_step)
    return fuel


print(sum(map(fuel_really_needed, masses)))
