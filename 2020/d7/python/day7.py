def get_colors(set, is_content=False):
    set = set.replace('.', ' ').replace(
        ',', '').replace('bags', 'bag').split(' bag')[:-1]
    return [(x.strip()[:2], x.strip()[2:]) for x in set] if is_content else set[0]


def prepare_rules(data):
    rules = {}
    for (container, content) in data:
        container = get_colors(container)
        content = get_colors(content, True)
        rules[container] = []
        for c in content:
            if c[0] == 'no':
                c = (0, '')
            rules[container].append({
                'n': int(c[0]),
                'color': c[1]
            })
    return rules


def contains_bag(rules, container):
    if container in rules:
        if any('shiny gold' in bag['color'] for bag in rules[container]):
            return True
        else:
            for subcontainer in rules[container]:
                if contains_bag(rules, subcontainer['color']):
                    return True
    return False


def nested_size(rules, container, init=False):
    k = container if init else container['color']
    s = 0 if init else 1
    if k in rules:
        content = rules[k]
        for bag in content:
            s += bag['n'] * nested_size(rules, bag)
        return s
    else:
        return container['n']


f = open('2020\\d7\\input.txt')
data = [l.strip().split(' contain ') for l in f.readlines()]
rules = prepare_rules(data)
can_contain = sum([contains_bag(rules, r) for r in rules])
ns = nested_size(rules, 'shiny gold', True)

print(f'{can_contain} different bag colors can contain a shiny gold bag')
print(f'You need {ns} bags inside your shiny gold bag')
