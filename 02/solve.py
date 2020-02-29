orig_program = list(map(int, open("02/input.txt").read().split(",")))
# program = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
# program = [1, 0, 0, 0, 99]
# program = [2, 3, 0, 3, 99]
# program = [2, 4, 4, 5, 99, 0]


def run_program(program):
    pc = 0
    while True:
        if program[pc] == 1:
            program[program[pc + 3]] = program[program[pc+1]] + \
                program[program[pc+2]]
            pc += 4
        elif program[pc] == 2:
            program[program[pc + 3]] = program[program[pc+1]] * \
                program[program[pc+2]]
            pc += 4
        elif program[pc] == 99:
            return program
        else:
            raise Exception("invalid opcode")


wanted = 19690720

for noun in range(100):
    for verb in range(100):
        program = orig_program.copy()
        program[1] = noun
        program[2] = verb
        output = run_program(program)[0]
        if output == wanted:
            print(
                f"noun = {noun}, verb = {verb}, result = {100 * noun + verb}")


# as stated in challenge
# print(run_program(program)[0])
