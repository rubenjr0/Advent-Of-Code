def get_challenge1_answer(entries):
    for i in range(0, len(entries)):
        for j in range(i + 1, len(entries)):
            if entries[i] + entries[j] == 2020:
                return entries[i] * entries[j]


def get_challenge2_answer(entries):
    for i in range(0, len(entries)):
        for j in range(i + 1, len(entries)):
            for k in range(j + 1, len(entries)):
                if entries[i] + entries[j] + entries[k] == 2020:
                    return entries[i] * entries[j] * entries[k]


f = open('2020\\d1\\input.txt')
entries = list(map(lambda x: int(x.strip()), f.readlines()))
answer1 = get_challenge1_answer(entries)
answer2 = get_challenge2_answer(entries)

print(answer1)
print(answer2)
