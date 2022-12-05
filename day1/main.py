# first part
def get_most_calories():
    with open("input", "r") as input:
        most_calories = 0
        lines = input.readlines()

        current_calories = 0

        for line in lines:
            if line.strip() == "":
                if current_calories > most_calories:
                    most_calories = current_calories

                current_calories = 0
                continue

            current_calories += int(line.strip())

        return most_calories

# second part
def get_top_three():
    with open("input", "r") as input:
        lines = input.readlines()

        calorie_sum = []
        current_calories = 0

        for line in lines:
            if line.strip() == "":
                calorie_sum.append(current_calories)

                current_calories = 0
                continue

            current_calories += int(line.strip())

        sorted_calorie_sum = sorted(calorie_sum, reverse=True)
        
        return sorted_calorie_sum[0] + sorted_calorie_sum[1] + sorted_calorie_sum[2]

if __name__ == "__main__":
    # first part
    calories = get_most_calories()
    print(calories)

    # second part
    top_three_sum = get_top_three()
    print(top_three_sum)
