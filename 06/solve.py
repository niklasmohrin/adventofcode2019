from collections import defaultdict
from queue import Queue

# class Planet:
#     def __init__(self, name):
#         self.name = name
#         self.orbit = []


orbits = open("06/input.txt").read().split("\n")[:-1]
orbits = [orbit.split(")") for orbit in orbits]


names = set(planet for orbit in orbits for planet in orbit)

# graph = dict((n, []) for n in names)
# graph = defaultdict(list)

# Part 1
# for orbit in orbits:
#     graph[orbit[0]].append(orbit[1])


# Part 1
# number_orbits = dict((n, 0) for n in names)
# number_orbits["COM"] = 0

# q = Queue()
# q.put("COM")

# total = 0

# while not q.empty():
#     planet = q.get()
#     total += number_orbits[planet]
#     for other in graph[planet]:
#         number_orbits[other] += 1 + number_orbits[planet]
#         q.put(other)

# print(total)

graph = dict()
for orbit in orbits:
    graph[orbit[1]] = orbit[0]

# figure out path from YOU to COM
cur = "YOU"
path = list()
while cur != "COM":
    cur = graph[cur]
    path.append(cur)

# see how many transfers it takes from SAN onto the path
cur = "SAN"
steps = 0
while cur not in path:
    cur = graph[cur]
    steps += 1

steps += path.index(cur) - 1

print(steps)
