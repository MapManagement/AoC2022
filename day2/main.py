POINTS = {"AY": 8, "AX": 4, "AZ": 3, "BY": 5, "BX": 1, "BZ": 9, "CY": 2, "CX": 7, "CZ": 6}
SIGN_POINTS = {"X": 1, "Y": 2, "Z": 3}
PART_2_MOVES = {
    "X": {
        "A": "Z",
        "B": "X",
        "C": "Y"
    },
    "Y": {
        "A": "X",
        "B": "Y",
        "C": "Z"
    },
    "Z": {
        "A": "Y",
        "B": "Z",
        "C": "X"
    }
}

def part1():
    with open("input", "r") as input:
        total_score = 0
        lines = input.readlines()
        # A = rock, B = paper, C = scissors
        # X1 = rock, Y2 = paper, Z3 = scissors

        for line in lines:
            game_moves = line.strip().replace(" ", "")
            total_score += POINTS[game_moves]

        return total_score

def part2():
    with open("input", "r") as input:
        total_score = 0
        lines = input.readlines()

        for line in lines:
            signs = line.split(" ")
            own_move = PART_2_MOVES[signs[1].strip()][signs[0]]
            game_moves = signs[0] + own_move

            total_score += POINTS[game_moves]

        return total_score
                    

if __name__ == "__main__":
    part1_score = part1()
    part2_score = part2()
    print(f"Part 1: {part1_score}")
    print(f"Part 2: {part2_score}")
