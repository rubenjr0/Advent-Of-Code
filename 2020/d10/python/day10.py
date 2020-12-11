def get_adapter_differences(adapters):
    current_jolts = 0
    differences = {
        1: 0,
        3: 1
    }
    for i in range(len(adapters)):
        diff = adapters[i] - current_jolts
        if diff == 1:
            differences[1] += 1
        elif diff == 3:
            differences[3] += 1
        current_jolts = adapters[i]
    return current_jolts + 3, differences


def combinations(adapters, visited, jolt, max_jolts):
    if jolt == max_jolts:
        return 1
    paths = 0
    if jolt + 1 in adapters:
        if jolt + 1 not in visited:
            visited[jolt +
                    1] = combinations(adapters, visited, jolt + 1, max_jolts)
        paths += visited[jolt + 1]
    if jolt + 2 in adapters:
        if jolt + 2 not in visited:
            visited[jolt +
                    2] = combinations(adapters, visited, jolt + 2, max_jolts)
        paths += visited[jolt + 2]
    if jolt + 3 in adapters:
        if jolt + 3 not in visited:
            visited[jolt +
                    3] = combinations(adapters, visited, jolt + 3, max_jolts)
        paths += visited[jolt + 3]
    return paths


f = open('2020\\d10\\input.txt')
adapters = sorted([int(a) for a in f.readlines()])

jolts, differences = get_adapter_differences(adapters)
visited = {}

print(
    f'The final output is {jolts} jolts, and the differences are: {differences}')
print(
    f'1-jolt differences * 3-jolt differences = {differences[1] * differences[3]}')

print(
    f'Possible combinations: {combinations(adapters + [jolts], visited, 0, jolts)}')
