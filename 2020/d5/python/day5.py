def decode(data, hi, hi_sym, lo_sym):
    lo = 0
    for x in data:
        mid = (hi + lo) // 2
        if x == lo_sym:
            hi = mid
        elif x == hi_sym:
            lo = mid
    return lo


def decode_pass(p):
    row = p[:7]
    col = p[7:]
    return (
        decode(row, 128, 'B', 'F'),
        decode(col, 8, 'R', 'L')
    )


def get_seat_ID(seat):
    return seat[0] * 8 + seat[1]


def get_continuous_ids(ids):
    ids = sorted(ids)
    for i in range(2, len(ids)):
        current_id, next_id = ids[i-2:i]
        if next_id != current_id + 1:
            return current_id + 1


f = open('2020\\d5\\input.txt')
passes = f.readlines()
passes = [p.strip() for p in passes]

seats = [decode_pass(p) for p in passes]
ids = [get_seat_ID(s) for s in seats]

print(f'The highest id is {max(ids)}')
print(f'Your seat has the id {get_continuous_ids(ids)}')
