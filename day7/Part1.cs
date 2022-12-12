namespace day7 
{
	public static class Part1
	{
		
		public static int GetSizeOfDeletedData()
		{
			var root = IterateActions();
			var totalSize = GetTotalSize(root, 0);

			// Part Two
			var smallestDir = Part2.GetTotalSizeOfDeletedDir(root);
			Console.WriteLine($"Smallest directory to delete: {smallestDir}");

			return totalSize;
		}

		private static string[] ReadInputLines()
		{
			string[] lines = System.IO.File.ReadAllLines("input");

			return lines;
		}

		private static int GetTotalSize(Directory root, int totalSize)
		{
			var currentDirectory = root;

			if (currentDirectory.SubDirectories.Count == 0)
				return totalSize;

			foreach (var dir in currentDirectory.SubDirectories)
			{
				if (dir.Size <= 100_000)
					totalSize += dir.Size;

				totalSize = GetTotalSize(dir, totalSize);
			}

			return totalSize;
		}

		private static Directory IterateActions()
		{
			var lines = ReadInputLines();
			var root = new Directory();
			var currentDirectory = root;

			foreach (var line in lines)
			{
				var action = ReadAction(line);

				if (action == Action.File)
				{
					ExecuteFilePath(currentDirectory, line);
				}
				else if (action == Action.Command)
				{
					currentDirectory = ExecuteDirectoryPath(currentDirectory, line);
				}
			}

			return root;
		}

		private static void ExecuteFilePath(Directory root, string line)
		{
			var currentDir = root;

			while (currentDir != null)
			{
				currentDir.Size += GetFileSize(line);
				currentDir = currentDir.ParentDirectory;
			}
		}

		private static Directory ExecuteDirectoryPath(Directory root, string line)
		{
			var direction = GetCommandDirection(line);

			if (direction == Direction.Up)
			{
				var parent = root.ParentDirectory;
				return parent;
			}
			else if (direction == Direction.Down)
			{
				var newDirectory = new Directory()
				{
					ParentDirectory = root,
					Name = line
				};

				root.SubDirectories.Add(newDirectory);

				return newDirectory;
			}

			return root;
		}

		private static Action ReadAction(string line) 
		{
			char firstChar = line[0];

			if (firstChar == '$')
			{
				return Action.Command;
			}
			else if (Char.IsDigit(firstChar))
			{
				return Action.File;
			}
			else
			{
				return Action.Directory;
			}
		}

		private static Direction GetCommandDirection(string commandLine)
		{
			var commandParts = commandLine.Split(" ");

			if (commandParts.Length == 2)
			{
				return Direction.Stay;
			}
			else if (commandParts[2].ToString() == "..")
			{
				return Direction.Up;
			}
			else 
			{
				return Direction.Down;
			}
		}


		private static int GetFileSize(string fileLine)
		{
			var fileParts = fileLine.Split(" ");

			if (fileParts.Length < 1)
			{
				return 0;
			}

			var fileSize = int.Parse(fileParts[0]);

			return fileSize;
		}
	}
}
