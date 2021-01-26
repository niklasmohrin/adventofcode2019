#!/usr/bin/env python3

data = open("14/input.txt").read().rstrip().split("\n")


class Needed:
    def __init__(self, kind, quantity):
        self.kind = kind
        self.quantity = int(quantity)


class Recipe:
    def __init__(self, line):
        res, req = parse_line(line)
        res_quan, res = res.split(" ")
        res_quan = int(res_quan)
        self.quantity = res_quan
        self.result = res

        req = [r.split(" ") for r in req]
        req = [Needed(r[1], r[0]) for r in req]
        self.requirements = req


def parse_line(line):
    req, res = line.split("=>")
    res = res.strip()
    req = [r.strip() for r in req.split(",")]
    return res, req


recipes = dict(map(lambda r: (r.result, r), map(Recipe, data)))


def ore_needed(mat, quantity):
    if mat == "ORE":
        return quantity
    else:
        s = 0
        for req in recipes[mat].requirements:
            s += ore_needed(req.kind, req.quantity)
        return s


print(ore_needed("FUEL", 1))
