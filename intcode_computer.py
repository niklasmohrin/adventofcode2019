#!/usr/bin/env python3

import sys

op_lens = {
    1: 4,
    2: 4,
    99: 1,
    3: 2,
    4: 2,
    5: 3,
    6: 3,
    7: 4,
    8: 4,
}


def run_program(program):
    pc = 0
    while True:
        # Opcode parsing
        opcode = program[pc]
        instruction = opcode % 100
        modes = [
            (opcode // 100) % 10,
            (opcode // 1000) % 10,
            (opcode // 10000) % 10,
        ]

        # mode parsing and parameter loading
        parameter_adrs = list()
        op_len = op_lens[instruction]
        for i in range(op_len - 1):
            if modes[i] == 0:
                parameter_adrs.append(program[pc + i + 1])
            elif modes[i] == 1:
                parameter_adrs.append(pc + 1 + i)
            else:
                raise

        pc_jumped = False

        # execution
        if instruction == 99:
            return program
        elif instruction == 1:
            p1 = program[parameter_adrs[0]]
            p2 = program[parameter_adrs[1]]
            program[parameter_adrs[2]] = p1 + p2
        elif instruction == 2:
            p1 = program[parameter_adrs[0]]
            p2 = program[parameter_adrs[1]]
            program[parameter_adrs[2]] = p1 * p2
        elif instruction == 3:
            val = int(input())
            program[parameter_adrs[0]] = val
        elif instruction == 4:
            val = program[parameter_adrs[0]]
            print(val)
        elif instruction == 5:
            p1 = program[parameter_adrs[0]]
            p2 = program[parameter_adrs[1]]
            if p1:
                pc_jumped = True
                pc = p2
        elif instruction == 6:
            p1 = program[parameter_adrs[0]]
            p2 = program[parameter_adrs[1]]
            if not p1:
                pc_jumped = True
                pc = p2
        elif instruction == 7:
            p1 = program[parameter_adrs[0]]
            p2 = program[parameter_adrs[1]]
            program[parameter_adrs[2]] = p1 < p2
        elif instruction == 8:
            p1 = program[parameter_adrs[0]]
            p2 = program[parameter_adrs[1]]
            program[parameter_adrs[2]] = p1 == p2
        else:
            raise

        if not pc_jumped:
            pc += op_len


if __name__ == "__main__":
    filename = sys.argv[1]
    program = list(map(int, open(filename).read().split(",")))

    run_program(program)
