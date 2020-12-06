def count_any_yes(g: str):
    y = 0
    for c in range(ord('a'), ord('z') + 1):
        if chr(c) in g:
            y += 1
    return y


def count_all_yes(g: list):
    n_persons = len(g)
    answers = ''.join(g)
    y = 0
    for c in range(ord('a'), ord('z') + 1):
        if answers.count(chr(c)) == n_persons:
            y += 1
    return y


f = open('2020\\d6\\input.txt')
data = f.read().split('\n\n')
groups = [d.replace(' ', '').replace('\n', ' ').split() for d in data]

any_sum = sum([count_any_yes(''.join(g)) for g in groups])
all_sum = sum([count_all_yes(g) for g in groups])

print(any_sum)
print(all_sum)
