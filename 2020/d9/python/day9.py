def find_pair_sum_to(preamble, n):
    for x in preamble:
        need = n - x
        if need in preamble and need != x:
            return (x, need)
    return None


def find_invalid_number(data):
    for i in range(25, len(data)):
        preamble = data[i - 25:i]
        n = data[i]
        pairs = find_pair_sum_to(preamble, n)
        if pairs == None:
            return n


def find_list_sum_to(preamble, n):
    for i in range(len(preamble) - 2):
        for j in range(2, len(preamble)):
            s = sum(preamble[i:j])
            if s > n:
                break
            elif s == n:
                return preamble[i:j]


f = open('2020\\d9\\input.txt')
data = [int(l) for l in f.readlines()]
invalid = find_invalid_number(data)
weakness_list = find_list_sum_to(data, invalid)
weakness = min(weakness_list) + max(weakness_list)
print(f'The invalid number is {invalid}')
print(f'The weakness of the encryption is {weakness}')
