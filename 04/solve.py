start, end = map(int, "158126-624574".split("-"))

start = max(start, 100000)
end = min(end, 999999)
found = 0

for n in range(start, end+1):
    double = False
    prev = n % 10
    adjacent = 1
    while n:
        n //= 10
        digit = n % 10
        if digit == prev:
            adjacent += 1
        elif digit > prev:
            break
        else:
            if adjacent == 2:
                double = True
            adjacent = 1
        prev = digit
    else:
        if double:
            found += 1

print(found)
