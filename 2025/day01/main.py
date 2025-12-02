lines = []
with open("input.txt", "r") as f:
    for line in f:
        lines.append(line.strip())

solution1 = 0
solution = 0
location = 50
for line in lines:
    match line[0]:
        case "L":
            rotation = int(line[1:])
            solution += rotation // 100
            rotation = rotation % 100
            if rotation >= location and location != 0:
                solution += 1
            location = (location + (100 - rotation)) % 100
        case "R":
            rotation = int(line[1:])
            solution += rotation // 100
            rotation = rotation % 100
            if rotation >= (100 - location) and location != 0:
                solution += 1
            location = (location + rotation) % 100
        case _:
            assert False
    if location == 0:
        solution1 += 1


print(solution1)
print(solution)
