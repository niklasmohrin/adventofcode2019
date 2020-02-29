orig_program = list(map(int, open("05/input.txt").read().split(",")))

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
        for i in range(op_len-1):
            if modes[i] == 0:
                parameter_adrs.append(program[pc+i+1])
            elif modes[i] == 1:
                parameter_adrs.append(pc+1+i)
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


# run_program([3, 0, 4, 0, 99])
# print(run_program([1002, 4, 3, 4, 33]))
# print(run_program([1101, 100, -1, 4, 0]))

# print(run_program([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]))
# print(run_program([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]))
# print(run_program([3, 3, 1108, -1, 8, 3, 4, 3, 99]))
# print(run_program([3, 3, 1107, -1, 8, 3, 4, 3, 99]))

# print(run_program([3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]))
# print(run_program([3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]))

# print(run_program([3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
#                    1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
#                    999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]))

run_program(orig_program)
