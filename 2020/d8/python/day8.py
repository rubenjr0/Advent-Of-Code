def run_program(program):
    visited = []
    PC = 0
    acc = 0
    abort = False
    while PC < len(program):
        if PC in visited:
            abort = True
            break
        i, n = program[PC]
        visited.append(PC)
        if i == 'jmp':
            PC += n
        else:
            if i == 'acc':
                acc += n
            PC += 1
    return acc, visited, abort


def find_fix(program, check):
    ogp = program
    for pc in check:
        program = ogp.copy()
        i, n = program[pc]
        if i == 'jmp' or i == 'nop':
            if i == 'jmp':
                program[pc] = ('nop', n)
            else:
                program[pc] = ('jmp', n)
            acc, _, aborted = run_program(program)
            if not aborted:
                print(
                    f'Error at {pc + 1} -> acc = {acc}')
                break


f = open('2020\\d8\\input.txt')
program = [i.split() for i in f.readlines()]
program = [(i, int(n)) for (i, n) in program]

acc, v, abort = run_program(program)
if not abort:
    print(f'ACC: {acc}')
else:
    find_fix(program, v)
