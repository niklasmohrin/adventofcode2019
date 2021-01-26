from itertools import cycle, repeat

# data = "80871224585914546619083218645595"
data = open("16/input.txt").read()

data = data.strip()
data = repeat(data, 10000)
data = list(map(int, data))


def base_pattern(repetitions):
    pattern = [0, 1, 0, -1]
    pattern = [val for val in pattern for _ in range(repetitions)]

    return pattern


def pattern_to_be_used(repetitions):
    pattern = base_pattern(repetitions)
    pattern = cycle(pattern)
    next(pattern)
    return pattern


def multiply(t):
    return t[0] * t[1]


def fft_phase(seq):
    res = []
    for i in range(1, len(seq) + 1):
        pattern = pattern_to_be_used(i)
        res.append(abs(sum(map(multiply, zip(seq, pattern)))) % 10)
    return res


def apply_fft_phases(seq, n):
    for _ in range(n):
        seq = fft_phase(seq)
    return seq


def seq_to_str(seq):
    return "".join(map(str, seq))


print(seq_to_str(apply_fft_phases(data, 100))[:8])
