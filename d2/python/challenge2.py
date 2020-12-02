def validate(entry):
    policy, char, password = entry
    i1, i2 = list(map(lambda p: int(p) - 1, policy.split('-')))
    return (password[i1] == char and not password[i2] == char) or (not password[i1] == char and password[i2] == char)


f = open('d2\\input.txt')
passwords = list(
    map(
        lambda x: x.replace(':', '').split(),
        f.readlines()
    )
)

print(len([x for x in passwords if validate(x)]))
