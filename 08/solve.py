data = list(map(int, open("08/input.txt").read().strip()))

w, h = 25, 6
layer_size = w * h
layers = []
for i in range(0, len(data), layer_size):
    layers.append(data[i:i + layer_size])


def part_one():
    def count_zeros(layer):
        return layer.count(0)

    layer = min(layers, key=count_zeros)
    print(layer.count(1) * layer.count(2))


def part_two():
    for y in range(h):
        for x in range(w):
            for layer in layers:
                if layer[x + y * w] == 0:
                    print(" ", end="")
                    break
                elif layer[x + y * w] == 1:
                    print("#", end="")
                    break
        print("\n", end="")


# part_one()
part_two()
