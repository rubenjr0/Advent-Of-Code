def challenge1_validation(entry):
    policy, char, password = entry
    min, max = list(map(lambda p: int(p), policy.split('-')))
    return min <= password.count(char) <= max


def challenge2_validation(entry):
    policy, char, password = entry
    i1, i2 = list(map(lambda p: int(p) - 1, policy.split('-')))
    return (password[i1] == char and not password[i2] == char) or (not password[i1] == char and password[i2] == char)


f = open('2020\\d2\\input.txt')
passwords = list(
    map(
        lambda x: x.replace(':', '').split(),
        f.readlines()
    )
)

answer1 = len([x for x in passwords if challenge1_validation(x)])
answer2 = len([x for x in passwords if challenge2_validation(x)])

print(answer1)
print(answer2)
