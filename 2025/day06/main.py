lines = []
raw_lines = []
with open("input.txt", "r") as f:
    for line in f.readlines():
        elements = line.split()
        elements = [element.strip() for element in elements]
        lines.append(elements)
        raw_lines.append(line)

problem_count = len(lines[0])
problem_size = len(lines)
output_sum = 0

# print(lines)
for i in range(problem_count):
    calc = int(lines[0][i])
    match lines[problem_size - 1][i]:
        case "*":
            for j in range(1, len(lines) - 1):
                calc *= int(lines[j][i])
        case "+":
            for j in range(1, len(lines) - 1):
                calc += int(lines[j][i])
    # print(calc)
    output_sum += calc

print(output_sum)

# Part 2
raw_lines.insert(0, raw_lines[-1])
raw_lines.pop()

output_sum = 0
running_sum = 0
operation = " "
# for line in raw_lines:
#    print(line)
for x_2 in range(len(raw_lines[0])):
    number = ""
    if raw_lines[0][x_2] in ["*", "+"]:
        operation = raw_lines[0][x_2]
        # print(running_sum)
        output_sum += running_sum
        for x_1 in range(1, len(raw_lines)):
            number += raw_lines[x_1][x_2]
        # print(operation)
        # print(number.strip())
        running_sum = int(number.strip())
    else:
        for x_1 in range(1, len(raw_lines)):
            # print(raw_lines[x_1][x_2])
            number += raw_lines[x_1][x_2]
        # print(number.strip())
        if operation == "+" and number.strip() != "":
            running_sum += int(number.strip())
        elif number.strip() != "":
            running_sum *= int(number.strip())


print(output_sum + running_sum)
