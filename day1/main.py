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

if __name__ == "__main__":
    calories = get_most_calories()
    print(calories)
