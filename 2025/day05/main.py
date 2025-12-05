ranges = []
numbers = []
before_split = False
with open("input.txt", "r") as f:
    for line in f:
        clean_line = line.strip()
        if clean_line.find("-") != -1:
            limits = clean_line.split("-")
            ranges.append([int(limits[0]), int(limits[1])])
        elif clean_line != "":
            numbers.append(int(clean_line))

fresh_count = 0
for number in numbers:
    for rangeA in ranges:
        if number >= int(rangeA[0]) and number <= int(rangeA[1]):
            fresh_count += 1
            # print(str(number) + str(rangeA))
            break

sorted_ranges = sorted(ranges, key=lambda x: x[0])
changed = True
while changed:
    # print(len(sorted_ranges))
    changed = False
    new_ranges = []
    skip = set()
    for i, limits in enumerate(sorted_ranges):
        if i in skip:
            continue
        merged = False
        for j in range(i + 1, len(sorted_ranges)):
            if limits[1] < sorted_ranges[j][0]:
                break
            if limits[1] >= sorted_ranges[j][0]:
                changed = True
                merged = True
                limits = [limits[0], max(limits[1], sorted_ranges[j][1])]
                skip.add(j)
        new_ranges.append(limits)
    sorted_ranges = new_ranges

fresh_total = 0
for limits in sorted_ranges:
    fresh_total += (limits[1] - limits[0]) + 1

print(fresh_count)
print(fresh_total)
