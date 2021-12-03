def read_file(fn):
    with open(fn) as f:
        return f.readlines()

def parse_input(input_data):
    return list(map(int, map(lambda line: line.strip(), input_data)))

def df(xs):
    dfs = []
    for i in range(1, len(xs)):
        dfs.append(xs[i] - xs[i-1])
    return dfs

def increases(data):
    return list(map(lambda v: int(v > 0), data))


def count_solutions(xs):
    return sum(increases(xs))

input_data = read_file('input')
data = parse_input(input_data)
dfs = df(data)
sol_1 = count_solutions(dfs)
print(sol_1)

windows = [data[i-3:i] for i in range(3, len(data) + 1)]
windows_sum = list(map(sum, windows))
dfs_2 = df(windows_sum)
sol_2 = count_solutions(dfs_2)
print(sol_2)
