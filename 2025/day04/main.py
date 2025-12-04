lines = []
with open("input.txt", "r") as f:
    for line in f:
        lines.append(list(line.strip()))

accessible_rolls = 0
for x_1, line in enumerate(lines):
    for x_2, element in enumerate(line):
        if element == "@":
            neighbors = 0
            # NW
            if x_1 > 0 and x_2 > 0 and lines[x_1 - 1][x_2 - 1] == "@":
                neighbors += 1
            # N
            if x_1 > 0 and lines[x_1 - 1][x_2] == "@":
                neighbors += 1
            # NE
            if x_1 > 0 and x_2 < len(line) - 1 and lines[x_1 - 1][x_2 + 1] == "@":
                neighbors += 1
            # E
            if x_2 < len(line) - 1 and lines[x_1][x_2 + 1] == "@":
                neighbors += 1
            # SE
            if (
                x_1 < len(line) - 1
                and x_2 < len(line) - 1
                and lines[x_1 + 1][x_2 + 1] == "@"
            ):
                neighbors += 1
            # S
            if x_1 < len(line) - 1 and lines[x_1 + 1][x_2] == "@":
                neighbors += 1
            # SW
            if x_1 < len(line) - 1 and x_2 > 0 and lines[x_1 + 1][x_2 - 1] == "@":
                neighbors += 1
            # W
            if x_2 > 0 and lines[x_1][x_2 - 1] == "@":
                neighbors += 1
            if neighbors < 4:
                accessible_rolls += 1

print(accessible_rolls)

cont = True
accessible_rolls = 0
while cont:
    cont = False
    for x_1, line in enumerate(lines):
        for x_2, element in enumerate(line):
            if element == "1":
                lines[x_1][x_2] = "."
                cont = True
            if element == "@":
                neighbors = 0
                # NW
                if x_1 > 0 and x_2 > 0 and lines[x_1 - 1][x_2 - 1] > ".":
                    neighbors += 1
                # N
                if x_1 > 0 and lines[x_1 - 1][x_2] > ".":
                    neighbors += 1
                # NE
                if x_1 > 0 and x_2 < len(line) - 1 and lines[x_1 - 1][x_2 + 1] > ".":
                    neighbors += 1
                # E
                if x_2 < len(line) - 1 and lines[x_1][x_2 + 1] > ".":
                    neighbors += 1
                # SE
                if (
                    x_1 < len(line) - 1
                    and x_2 < len(line) - 1
                    and lines[x_1 + 1][x_2 + 1] > "."
                ):
                    neighbors += 1
                # S
                if x_1 < len(line) - 1 and lines[x_1 + 1][x_2] > ".":
                    neighbors += 1
                # SW
                if x_1 < len(line) - 1 and x_2 > 0 and lines[x_1 + 1][x_2 - 1] > ".":
                    neighbors += 1
                # W
                if x_2 > 0 and lines[x_1][x_2 - 1] > ".":
                    neighbors += 1
                if neighbors < 4:
                    accessible_rolls += 1
                    cont = True
                    lines[x_1][x_2] = "1"

print(accessible_rolls)
