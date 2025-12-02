import math


def int_length(x: int):
    return int(math.log10(x)) + 1


def find_repeats_part_1(x: int, y: int):
    assert x < y
    # Split if different lengths
    length_x = int_length(x)
    length_y = int_length(y)
    if length_x < length_y:
        return find_repeats_part_1(x, 10**length_x - 1) + find_repeats_part_1(
            10**length_x, y
        )
    # Return 0 if length is odd since repeats are impossible
    elif length_x % 2 == 1:
        return 0
    # Get all "repeatable prefixes"
    else:
        first_half_x = int(str(x)[: length_x // 2])
        first_half_y = int(str(y)[: length_x // 2])
        return_sum = 0
        for prefix in range(first_half_x, first_half_y + 1):
            number = int(str(prefix) + str(prefix))
            if number >= x and number <= y:
                return_sum += number
        return return_sum


def find_repeats_part_2(x: int, y: int):
    assert x < y
    # Split if different lengths
    length_x = int_length(x)
    length_y = int_length(y)
    if length_x < length_y:
        return find_repeats_part_2(x, 10**length_x - 1).union(
            find_repeats_part_2(10**length_x, y)
        )
    # Get all "repeatable prefixes"
    else:
        return_set = set()
        for repeat_length in range(1, (length_x // 2) + 1):
            if length_x % repeat_length != 0:
                continue
            else:
                element_x = int(str(x)[:repeat_length])
                element_y = int(str(y)[:repeat_length])
                repeats = length_x // repeat_length
                for prefix in range(element_x, element_y + 1):
                    number = int(str(prefix) * repeats)
                    if number >= x and number <= y:
                        return_set.add(number)
        return return_set


lines = []
with open("input.txt", "r") as f:
    for line in f:
        lines.append(line.strip())

input = lines[0]

inputs = input.split(",")
result1 = 0
result2 = 0
for ranges in inputs:
    limits = ranges.split("-")
    result1 += find_repeats_part_1(int(limits[0]), int(limits[1]))
    result2 += sum(find_repeats_part_2(int(limits[0]), int(limits[1])))
print(result1)
print(result2)
