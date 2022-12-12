namespace day7
{
	public static class Part2
	{
		private const int MaxSpace = 70_000_000;
		private const int NeededSpace = 30_000_000;

		public static int GetTotalSizeOfDeletedDir(Directory root)
		{
			var usedSpace = root.Size;
			var unusedSpace = MaxSpace - usedSpace;
			var spaceToBeCleared = NeededSpace - unusedSpace;
			
			var sizes = GetFittingDirectories(root, new List<int>(), spaceToBeCleared);
			var smallestDirectory = GetSmallestDirectory(sizes);

			return smallestDirectory;
		}

		private static List<int> GetFittingDirectories(Directory root, List<int> directories, int spaceToBeCleared)
		{
			var currentDirectory = root;
			
			if (currentDirectory.SubDirectories.Count == 0)
				return directories;

			foreach (var dir in currentDirectory.SubDirectories)
			{
				if (dir.Size >= spaceToBeCleared)
						directories.Add(dir.Size);

				directories = GetFittingDirectories(dir, directories, spaceToBeCleared);
			}

			return directories;
		}

		private static int GetSmallestDirectory(List<int> directorySizes)
		{
			if (directorySizes.Count == 0)
				return 0;

			int smallestDirectory = directorySizes[0];

			foreach (var size in directorySizes)
			{
				if (size < smallestDirectory)
					smallestDirectory = size;
			}

			return smallestDirectory;
		}
	}
}
