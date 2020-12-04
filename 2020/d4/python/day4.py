import re


def between(low, n, high):
    return low <= n and n <= high


def check_hgt(value):
    if 'cm' in value or 'in' in value:
        height = int(value[:-2])
        unit = value[-2:]
        if unit == 'cm':
            return between(150, height, 193)
        elif unit == 'in':
            return between(59, height, 76)
    return False


def check_field(key, value):
    if key == 'byr':
        return between(1920, int(value), 2002)
    elif key == 'iyr':
        return between(2010, int(value), 2020)
    elif key == 'eyr':
        return between(2020, int(value), 2030)
    elif key == 'hgt':
        return check_hgt(value)
    elif key == 'hcl':
        return bool(re.compile('^#([a-f]|[0-9]){6}$').match(value))
    elif key == 'ecl':
        return value in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']
    elif key == 'pid':
        return bool(re.compile('[0-9]{9}$').match(value))


def validate_passport(passport):
    passport_fields = passport.split()
    keys = [field.split(':')[0] for field in passport_fields]
    values = [field.split(':')[1] for field in passport_fields]
    if not all([k in keys for k in ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']]):
        return False
    for (key, value) in zip(keys, values):
        if key != 'cid' and not check_field(key, value):
            return False
    return True


f = open('2020\\d4\\input.txt')
data = f.read().split('\n\n')
passports = [d.replace('\n', ' ') for d in data]

print(f'{len([passport for passport in passports if validate_passport(passport)])} valid passports')
