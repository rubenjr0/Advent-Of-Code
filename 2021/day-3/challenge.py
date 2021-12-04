def read_file(fn):
    with open(fn) as f:
        return list(map(lambda line: line.strip(), f.readlines()))


def get_column(data, index):
    col = []
    for d in data:
        col.append(d[index])
    return col


def get_all_columns(data):
    columns = []
    cols = len(data[0])
    for i in range(cols):
        columns.append(get_column(data, i))
    return columns


def get_common_bit(bits, crit=lambda ones, n: 1 if ones >= n else 0):
    n = len(bits) / 2
    ones = 0
    for c in bits:
        if c == '1':
            ones += 1
            if ones > n:
                break
    return crit(ones, n)


def bin_to_dec(bits, inv=False):
    n = len(bits)
    num = 0
    for i in range(n):
        mlt = (1-bits[i] if inv else bits[i])
        q = mlt * 2**(n - 1 - i)
        num += q
    return num


data = read_file('2021/day-3/input')
cols = get_all_columns(data)

most_common_bits = list(map(get_common_bit, cols))

gamma = bin_to_dec(most_common_bits)
epsilon = bin_to_dec(most_common_bits, True)
print(f'Part One: {gamma * epsilon}')


def get_life_support(data, crit):
    lfsp = []
    cb = get_common_bit(get_column(data, 0), crit)
    for d in data:
        if int(d[0]) == cb:
            lfsp.append(d)
    i = 1
    while len(lfsp) > 1:
        cb = get_common_bit(get_column(lfsp, i), crit)
        new_lfsp = []
        for d in lfsp:
            if int(d[i]) == cb:
                new_lfsp.append(d)
        if len(new_lfsp) == 1:
            return int(new_lfsp[0], 2)
        else:
            lfsp = new_lfsp
            i += 1


oxygen = get_life_support(data, lambda ones, n: 1 if ones >= n else 0)
co2 = get_life_support(data, lambda ones, n: 1 if ones < n else 0)

print('Oxygen:', oxygen)
print('CO2:', co2)

print(f'Part Two: {oxygen * co2}')
