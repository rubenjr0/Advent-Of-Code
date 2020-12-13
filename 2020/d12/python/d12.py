def final_position_part_1(instructions):
    x = 0
    y = 0
    angle = 0
    for d, a in instructions:
        if d == 'E' or d == 'F' and angle == 0:
            x += a
        elif d == 'N' or d == 'F' and angle == 90:
            y += a
        elif d == 'W' or d == 'F' and angle == 180:
            x -= a
        elif d == 'S' or d == 'F' and angle == 270:
            y -= a
        else:
            if d == 'L':
                angle += a
            elif d == 'R':
                angle += 360 - a if angle - a < 0 else -a
            angle %= 360
    return (x, y)


def turn(coords, dir, a):
    x, y = coords
    if a == 180:
        x, y = -x, -y
    elif dir == 'R' and a == 90 or dir == 'L' and a == 270:
        x, y = y, -x
    elif dir == 'L' and a == 90 or dir == 'R' and a == 270:
        x, y = -y, x
    return (x, y)


def waypoint(x, y, instruction):
    d, a = instruction
    if d == 'N':
        y += a
    elif d == 'S':
        y -= a
    elif d == 'E':
        x += a
    elif d == 'W':
        x -= a
    else:
        x, y = turn((x, y), d, a)
    return (x, y)


def step_ship(x, y, waypoint, a):
    wx, wy = waypoint
    return x + wx * a, y + wy * a


def manhattan_distance(position):
    return sum(map(abs, position))


f = open('2020\\d12\\input.txt')
instructions = [(l[0], int(l[1:])) for l in f.readlines()]
print(
    f'Manhattan distance in part 1: {manhattan_distance(final_position_part_1(instructions))}')

wx, wy = 10, 1
sx = sy = 0
for d, a in instructions:
    if d == 'F':
        sx, sy = step_ship(sx, sy, (wx, wy), a)
    else:
        wx, wy = waypoint(wx, wy, (d, a))
print(f'Manhattan distance in part 2: {manhattan_distance((sx, sy))}')
