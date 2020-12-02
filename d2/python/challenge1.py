def validate(entry):
    min, max = list(map(lambda p: int(p), entry[0].split('-')))
    char = entry[1]
    password = entry[2]
    return min <= password.count(char) <= max


f = open('d2\\input.txt')
passwords = list(
    map(
        lambda x: x.replace(':', '').split(),
        f.readlines()
    )
)

print(len([x for x in passwords if validate(x)]))
