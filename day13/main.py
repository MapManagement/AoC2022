import ast

def read_input():
    brackets_pair = [[]]
    with open("input", "r") as f:
        lines = f.readlines()
        index = 0

        for line in lines:
            if len(line.strip()) == 0:
                index += 1
                brackets_pair.append([])
                continue

            brackets_pair[index].append(ast.literal_eval(line))

    return brackets_pair

def calculate_pair(pair) -> bool:
    if len(pair) == 0:
        return False

    pair1 = pair[0]
    pair2 = pair[1]

    is_ordered = compare_pair(pair1, pair2)

    return is_ordered

def compare_pair(left, right) -> bool:
    length = left if type(left) == int else len(left)

    """if len(right) < len(left) and (len(left) != 1 and len(right) != 1):
        return False

    if len(right) == 0:
        return False"""

    for i in range(length):
        l = left[i] if i < len(left) else left[len(left) - 1]
        r = right[i] if i < len(right) else right[len(right) - 1]
        print(l)
        print(r)
        print("---")

        """if i < len(left):
            l = left[i]
        else:
            l = left[len(left) - 1]

        if i < len(right):
            r = right[i]
        else:
            right[len(right) - 1]"""

        if type(l) == int and type(r) == int:
            if l > r:
                return False
        elif type(l) == int and type(r) == list:
            return compare_pair([l], r)
        elif type(l) == list and type(r) == int:
            return compare_pair(l, [r])
        elif type(l) == list and type(r) == list:
            for j in map(compare_pair, l, r):
                if j:
                    return True
            return compare_pair(len(l), len(r))

    return True

def part1() -> int:
    pairs = read_input()
    index = 1
    sum = 0

    for pair in pairs:
        is_ordered = calculate_pair(pair)
        #print(is_ordered)
        if is_ordered:
            #print(index)
            sum += index

        index += 1

    return sum

if __name__ == "__main__":
    p1 = part1()
    print(f"Part 1: {p1}")
