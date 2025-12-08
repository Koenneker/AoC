boxes = []

with open("input.txt", "r") as f:
    for lines in f.readlines():
        x, y, z = lines.split(",")
        boxes.append((int(x), int(y), int(z)))


def distance(a, b):
    return ((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 + (a[2] - b[2]) ** 2) ** 0.5


# we calculate:
# n = 10/1000 shortest distance pairs

# we want:
# size of the three largest circuits made up by these pairsf

distances = {}
max_dist = 999999
merge_count = 1000
for i, box in enumerate(boxes):
    for j in range(i + 1, len(boxes)):
        dist = distance(box, boxes[j])
        if dist <= max_dist:
            distances[(box), boxes[j]] = distance(box, boxes[j])
            if len(distances) > merge_count:
                max_val = max(distances.values())
                distances = {
                    key: val for key, val in distances.items() if val != max_val
                }
                max_dist = max(distances.values())

connections = {}
for box in boxes:
    singleton_set = set()
    singleton_set.add(box)
    connections[box] = singleton_set

for distance_pair in distances.keys():
    if connections[distance_pair[0]] != connections[distance_pair[1]]:
        union = connections[distance_pair[0]].union(connections[distance_pair[1]])
        for element in union:
            connections[element] = union


sizes = list(map(len, set(map(frozenset, connections.values()))))
sizes.sort()
print("Part 1:")
print(sizes[-1] * sizes[-2] * sizes[-3])

distances = {}
max_dist = 999999
for i, box in enumerate(boxes):
    for j in range(i + 1, len(boxes)):
        dist = distance(box, boxes[j])
        distances[(box), boxes[j]] = distance(box, boxes[j])

for pairs, dist in iter(sorted(distances.items(), key=lambda pair: pair[1])):
    if connections[pairs[0]] != connections[pairs[1]]:
        union = connections[pairs[0]].union(connections[pairs[1]])
        if len(union) == len(boxes):
            print("Part 2:")
            print(pairs[0][0] * pairs[1][0])
            break
        for element in union:
            connections[element] = union
