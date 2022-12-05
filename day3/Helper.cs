namespace day3
{
	public static class Helper
	{
		public static int GetTypeSum()
		{
			string[] rucksacks = System.IO.File.ReadAllLines("input");
			int totalScore = 0;

			foreach (string rucksack in rucksacks)
			{
				totalScore += GetRucksackScore(rucksack);
			}

			return totalScore;
		}

		private static int GetRucksackScore(string rucksack)
		{
			int rucksackLength = rucksack.Length;
			string firstCompartment = rucksack.Substring(0, rucksackLength / 2);
			string secondCompartment = rucksack.Substring(rucksackLength / 2, rucksackLength / 2);

			for (int i = 0; i < firstCompartment.Length; i++)
			{
				char firstLetter = firstCompartment[i];

				for (int j = 0; j < secondCompartment.Length; j++)
				{
					char secondLetter = secondCompartment[j];

					if (firstLetter.Equals(secondLetter))
					{
						return CalculateLetterScore(firstLetter);
					}
				}
			}

			return 0;
		}

		private static int CalculateLetterScore(char letter)
		{
			if (!char.IsAscii(letter) || !char.IsLetter(letter))
				return 0;

			int value = (int)letter;

			if (char.IsUpper(letter))
			{
				return value - 38;
			}
			else if (char.IsLower(letter))
				return value - 96;

			return 0;
		}
	}
}
