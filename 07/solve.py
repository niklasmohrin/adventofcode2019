#!/usr/bin/env python3

from itertools import permutations
import os
import subprocess
import sys

filename = "07/input.txt"

max_signal = 0

for setting in permutations([0, 1, 2, 3, 4], 5):
    signal = 0
    for phase in setting:
        p = subprocess.run(["./intcode_computer.py", filename], stdout=subprocess.PIPE, input=f"{phase}\n{signal}\n", encoding="ascii")
        signal = int(p.stdout.rstrip())
    if signal > max_signal:
        # print(signal)
        max_signal = signal

print(max_signal)
