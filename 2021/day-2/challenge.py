def read_file(fn):
    with open(fn) as f:
        return f.readlines()


def parse_file(contents):
    lines = list(map(lambda line: line.strip().split(), contents))
    return list(map(lambda dir_qty: (dir_qty[0], int(dir_qty[1])), lines))


def run_course(course):
    position = 0
    depth = 0
    for (direction, quantity) in course:
        if direction == 'forward':
            position += quantity
        elif direction == 'up':
            depth -= quantity
        elif direction == 'down':
            depth += quantity
        else:
            raise 'Unexpected direction'
    return position * depth


def run_course_with_aim(course):
    position = 0
    depth = 0
    aim = 0
    for (direction, quantity) in course:
        if direction == 'forward':
            position += quantity
            depth += quantity * aim
        elif direction == 'up':
            aim -= quantity
        elif direction == 'down':
            aim += quantity
        else:
            raise 'Unexpected direction'
    return position * depth


fn = 'input'

contents = read_file(fn)
course = parse_file(contents)

print(f'Part One: {run_course(course)}')
print(f'Part Two: {run_course_with_aim(course)}')
