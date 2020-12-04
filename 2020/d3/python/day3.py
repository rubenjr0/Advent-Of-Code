def count_trees(data, slope, y, x, tree_squares):
    width = len(data[0])
    height = len(data)
    y_check = y + slope[1]
    if y_check >= height:
        return tree_squares
    else:
        x_check = (x + slope[0]) % width
        if data[y_check][x_check] == '#':
            tree_squares += 1
        return count_trees(data, slope, y_check, x_check, tree_squares)


f = open('2020\\d3\\input.txt')
data = [l.strip() for l in f.readlines()]
slopes = [(x, 1) for x in [1, 3, 5, 7]] + [(1, 2)]

prod = 1
trees = []
for slope in slopes:
    trees.append(count_trees(data, slope, 0, 0, 0))
    prod *= trees[-1]
    print(f'Encoutered {trees[-1]} trees using the slope {slope}')
print(prod)
