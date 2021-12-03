import time


def read_file(fn):
    with open(fn) as f:
        return f.readlines()


def parse_file(contents):
    lines = list(map(lambda line: line.strip().split(), contents))
    return list(map(lambda dir_qty: (dir_qty[0], int(dir_qty[1])), lines))


def optimize_course(course):  # Not needed, just for fun
    optimized = []
    for (direction, quantity) in course:
        if len(optimized) == 0:
            optimized.append((direction, quantity))
        else:
            opt_direction, opt_quantity = optimized[-1]
            if direction == opt_direction:
                optimized_quantity = 0
                if direction == 'forward' or direction == 'down':
                    optimized_quantity = opt_quantity + quantity
                elif direction == 'up':
                    optimized_quantity = opt_quantity - quantity
                if optimized_quantity == 0:
                    optimized = optimized[:-1]
                else:
                    optimized[-1] = (direction, optimized_quantity)
            else:
                optimized.append((direction, quantity))
    return optimized


def save_course(course):
    with open('optimized_input', 'w+') as f:
        for (direction, quantity) in course:
            f.write(f'{direction} {quantity}\n')


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
    return (position, depth)


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
    return (position, depth)


def repl():  # Not needed, just for fun
    x = 0
    a = 0
    d = 0
    while True:
        instruction = input('REPL /> ')
        direction, quantity = instruction.strip().split()
        quantity = int(quantity)
        if direction == 'forward':
            x += quantity
            d += quantity * a
        elif direction == 'up':
            a -= quantity
        elif direction == 'down':
            a += quantity
        else:
            raise 'wtf u doin bruh'
        print(f'> x: {x}\ty: {d}\ta: {a}\tp:{x*d}')


fn = 'input'

contents = read_file(fn)
course = parse_file(contents)

benchmarks = {}

start = time.perf_counter_ns()
optimized_course = optimize_course(course)
save_course(optimized_course)
end = time.perf_counter_ns()
benchmarks['optimize'] = end - start


start = time.perf_counter_ns()
final_position, final_depth = run_course(course)
end = time.perf_counter_ns()
benchmarks['basic run'] = end - start

start = time.perf_counter_ns()
final_position, final_depth = run_course(optimized_course)
end = time.perf_counter_ns()
benchmarks['optimized run'] = end - start

print('Part One :: No aim')
print(f'x: {final_position}, y: {final_depth}')
print(f'product: {final_position * final_depth}')

start = time.perf_counter_ns()
final_position, final_depth = run_course_with_aim(course)
end = time.perf_counter_ns()
benchmarks['basic aim run'] = end - start

start = time.perf_counter_ns()
final_position, final_depth = run_course_with_aim(optimized_course)
end = time.perf_counter_ns()
benchmarks['opt aim run'] = end - start

print('\nPart Two :: With aim')
print(f'x: {final_position}, y: {final_depth}')
print(f'product: {final_position * final_depth}\n')

for benchmark in benchmarks:
    print(f'{benchmark}\t{benchmarks[benchmark]} nanoseconds')

# repl()
