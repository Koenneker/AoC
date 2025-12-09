import numpy as np
from tqdm import tqdm

points = []
max_x = 0
max_y = 0
with open("input.txt") as f:
    for line in f.readlines():
        x, y = line.strip().split(",")
        max_x = max(int(x), max_x)
        max_y = max(int(y), max_y)
        points.append((int(x), int(y)))

# Part 1
max_square = 0
for i, (x1, y1) in enumerate(points):
    for j in range(i, len(points)):
        (x2, y2) = points[j]
        new_square = (max(x1, x2) - min(x1, x2) + 1) * (max(y1, y2) - min(y1, y2) + 1)
        max_square = max(max_square, new_square)

print(max_square)

# Part 2

grid = np.zeros((max_y + 1, max_x + 1), dtype=np.uint8)

# print(max_x)
# print(max_y)
# for line in grid:
#    print(line)

# Add corners
points_arr = np.array(points)
grid[points_arr[:, 1], points_arr[:, 0]] = 1

# for line in grid:
#    print(line)

# Fill lines
for i in range(len(points)):
    x1, y1 = points[i - 1]
    x2, y2 = points[i]
    if x1 == x2:  # vertical line
        grid[min(y1, y2) : max(y1, y2) + 1, x1] = 2
    else:  # horizontal line
        grid[y1, min(x1, x2) : max(x1, x2) + 1] = 2


# print("")
# for line in grid:
#    print(line)

# Fill grid

vertical_edges = np.zeros_like(grid, dtype=bool)
for i in tqdm(range(len(points))):
    x1, y1 = points[i - 1]
    x2, y2 = points[i]
    if x1 == x2:
        min_y, max_y = min(y1, y2), max(y1, y2)
        vertical_edges[min_y:max_y, x1] = True

for i in tqdm(range(grid.shape[0])):
    crossings = vertical_edges[i]
    inside = np.cumsum(crossings) % 2 == 1
    grid[i, (grid[i] == 0) & inside] = 3

# print("")
# for line in grid:
#    print(line)

# max_square = 0
# print("Checking pairs")
# for i, (x1, y1) in tqdm(enumerate(points)):
#    for j in range(i, len(points)):
#        (x2, y2) = points[j]
#        new_square = abs(x1 - x2 + 1) * abs(y1 - y2 + 1)
#        if new_square > max_square:
#            subgrid = grid[min(y1, y2) : max(y1, y2) + 1, min(x1, x2) : max(x1, x2) + 1]
#            if subgrid.min() != 0:
#                max_square = new_square

# Calculate valid ranges within each row
# valid_ranges = []
# for line in tqdm(grid):
#    row_ranges = []
#    start = None
#    for i, cell in enumerate(line):
#        if cell != 0 and start is None:
#            start = i
#       elif cell == 0 and start is not None:
#            row_ranges.append((start, i - 1))
#            start = None
#    if start is not None:
#        row_ranges.append((start, len(line) - 1))
#    valid_ranges.append(row_ranges)

valid_ranges = []
for line in tqdm(grid):
    nonzero = line != 0  # Create mask
    padded = np.concatenate([[False], nonzero, [False]])  # Prevent out of bounds
    starts = np.where(padded[1:] & ~padded[:-1])[0]  # Basically rising edge detection
    ends = np.where(~padded[1:] & padded[:-1])[0] - 1  # Falling edge detection
    valid_ranges.append(
        list(zip(starts, ends))
    )  # Since rising and falling edges alternate, we can zip to get our bounds


max_square = 0
for i, (x1, y1) in tqdm(enumerate(points)):
    for j in range(i, len(points)):
        (x2, y2) = points[j]
        new_square = (max(x1, x2) - min(x1, x2) + 1) * (max(y1, y2) - min(y1, y2) + 1)
        if new_square > max_square:
            min_x, max_x = min(x1, x2), max(x1, x2)
            min_y, max_y = min(y1, y2), max(y1, y2)

            valid = all(
                any(s <= min_x and e >= max_x for s, e in valid_ranges[y])
                for y in range(min_y, max_y + 1)
            )

            if valid:
                max_square = new_square

print(max_square)
