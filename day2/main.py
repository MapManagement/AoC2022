def calculate_total_score():
    with open("input", "r") as input:
        total_score = 0
        lines = input.readlines()
        # A = rock, B = paper, C = scissors
        # X1 = rock, Y2 = paper, Z3 = scissors
        points = {"AY": 8, "AX": 4, "AZ": 3, "BY": 5, "BX": 1, "BZ": 9, "CY": 2, "CX": 7, "CZ": 6}

        for line in lines:
            game_round = line.strip().replace(" ", "")
            total_score += points[game_round]

        return total_score

if __name__ == "__main__":
    score = calculate_total_score()
    print(score)
