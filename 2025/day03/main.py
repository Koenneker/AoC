lines = []
with open("input.txt", "r") as f:
    for line in f:
        lines.append(line.strip())

total_joltage = 0
for line in lines:
    last_battery = -1
    max_joltage = -1
    for battery in line:
        joltage = last_battery * 10 + int(battery)
        max_joltage = max(max_joltage, joltage)
        last_battery = max(last_battery, int(battery))
    total_joltage += max_joltage

print(total_joltage)


def remove_n(current_batteries: str, battery: str, n: int):
    return current_batteries[:n] + current_batteries[n + 1 :] + battery


total_joltage = 0
for line in lines:
    batteries = "0" * 12
    max_joltage = -1
    for battery in line:
        for i in range(0, 12):
            potential_batteries = remove_n(batteries, battery, i)
            if int(potential_batteries) > max_joltage:
                max_joltage = int(potential_batteries)
                batteries = potential_batteries
                break
    print(batteries)
    total_joltage += max_joltage


print(total_joltage)
