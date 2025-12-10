from collections import deque

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, milp
from tqdm import tqdm

machines = []
with open("input.txt") as f:
    for line in f.readlines():
        splits = line.strip().split(" ")
        lights = tuple(splits[0][1:-1])
        buttons = []
        for i in range(1, len(splits) - 1):
            buttons.append(splits[i][1:-1].split(","))
        jolts = tuple(map(int, splits[-1][1:-1].split(",")))
        machines.append([lights, buttons, jolts])


def solve_machine(machine):
    lights, buttons, jolts = machine
    n = len(lights)
    start = tuple("." * n)

    if start == lights:
        return 0

    visited = {start}
    queue = deque([(start, 0)])

    while queue:
        state, presses = queue.popleft()
        for button in buttons:
            new_state = list(state)
            for flip in button:
                idx = int(flip)
                new_state[idx] = "#" if new_state[idx] == "." else "."

            new_tuple = tuple(new_state)
            if new_tuple == lights:
                return presses + 1
            if new_tuple not in visited:
                visited.add(new_tuple)
                queue.append((new_tuple, presses + 1))

    assert False


def solve_jolts(machine):
    ligths, buttons, jolts = machine
    n = len(jolts)
    start = tuple([0] * n)

    if start == jolts:
        return 0

    visited = {start}
    queue = deque([(start, 0)])

    while queue:
        state, presses = queue.popleft()
        for button in buttons:
            new_state = list(state)
            for flip in button:
                idx = int(flip)
                new_state[idx] += 1

            new_tuple = tuple(new_state)
            if new_tuple == jolts:
                return presses + 1
            valid = True
            for a, b in zip(new_tuple, jolts):
                if a > int(b):
                    valid = False
            if new_tuple not in visited and valid:
                visited.add(new_tuple)
                queue.append((new_tuple, presses + 1))

    assert False


def solve_jolts_milp(machine):
    lights, buttons, jolts = machine
    A = np.zeros((len(lights), len(buttons)), dtype=float)
    for j, button in enumerate(buttons):
        for flip in button:
            A[int(flip), j] = 1.0

    b = np.array(jolts, dtype=float)

    c = np.ones(len(buttons))
    result = milp(
        c,
        constraints=LinearConstraint(A, lb=b, ub=b),
        integrality=np.ones(len(buttons)),
        bounds=Bounds(lb=0, ub=np.inf),
    )
    return int(round(result.fun))


total_presses = 0
total_presses2 = 0
for machine in machines:
    result = solve_machine(machine)
    total_presses += result
    result2 = solve_jolts_milp(machine)
    total_presses2 += result2

print(total_presses)
print(total_presses2)
