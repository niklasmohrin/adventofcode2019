#!/usr/bin/env python3.8

import sys
from string import ascii_lowercase, ascii_uppercase
from collections import defaultdict
import numpy as np
from queue import Queue
from math import inf

maze = open(sys.argv[1]).read().split("\n")[:-1]
H = len(maze)
W = len(maze[0])

for i in range(H):
    maze[i] = list(maze[i])

maze = np.array(maze)

doors = dict()
keys = dict()


# Find important stuff on the map
for y, row in enumerate(maze):
    for x, cell in enumerate(row):
        if cell == "@":
            start = y,x
        elif cell in ascii_lowercase:
            keys[cell] = y,x
        elif cell in ascii_uppercase:
            doors[cell] = y,x

# Calc distances for every cell and find out which doors need to be unlocked for which keys
dist_from_start = np.full((H, W), -1, dtype=int)
dist_from_start[start] = 0

q = Queue()
q.put(start)

directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]

while not q.empty():
    cur = q.get()
    print(cur)
    dist = dist_from_start[cur]
    for direc in directions:
        neigh = (cur[0] + direc[0], cur[1] + direc[1])
        try:
            if maze[neigh] != "#":
                if dist_from_start[neigh] == -1:
                    q.put(neigh)
                    dist_from_start[neigh] = dist + 1
        except:
            print(f"{neigh} failed")

print(maze)
print(dist_from_start)
