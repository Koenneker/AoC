lines = []
with open("input.txt", "r") as f:
    for line in f.readlines():
        lines.append(list(line.strip()))


def print_field(field):
    for line in field:
        print(line)


beam_indices = set()
num_of_splits = 0
for x_1, line in enumerate(lines):
    for x_2, char in enumerate(line):
        match char:
            case "S":
                beam_indices.add(x_2)
            case "^":
                if x_2 in beam_indices:
                    num_of_splits += 1
                    beam_indices.remove(x_2)
                    beam_indices.add(x_2 - 1)
                    beam_indices.add(x_2 + 1)
    for beam_index in beam_indices:
        lines[x_1][int(beam_index)] = "|"

print_field(lines)
print(num_of_splits)

with open("input.txt", "r") as f:
    for line in f.readlines():
        lines.append(list(line.strip()))

beam_indices = {}
num_of_splits = 0
for x_1, line in enumerate(lines):
    for x_2, char in enumerate(line):
        match char:
            case "S":
                beam_indices[x_2] = 1
            case "^":
                if x_2 in beam_indices.keys():
                    num_of_splits += 1
                    paths = beam_indices[x_2]
                    if x_2 - 1 in beam_indices:
                        beam_indices[x_2 - 1] = beam_indices[x_2 - 1] + paths
                    else:
                        beam_indices[x_2 - 1] = paths
                    if x_2 + 1 in beam_indices:
                        beam_indices[x_2 + 1] = beam_indices[x_2 + 1] + paths
                    else:
                        beam_indices[x_2 + 1] = paths
                    del beam_indices[x_2]
    for beam_index in beam_indices:
        lines[x_1][int(beam_index)] = beam_indices[beam_index]

sum = 0
for element in lines[-1]:
    if element != ".":
        sum += int(element)

print_field(lines)
print(sum)
