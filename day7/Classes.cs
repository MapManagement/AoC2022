namespace day7
{
	public class Directory
	{
		public Directory() 
		{
			Size = 0;
			ParentDirectory = null;
			SubDirectories = new List<Directory>();
		}

		public int Size { get; set; }
		public Directory? ParentDirectory { get; set; }
		public List<Directory> SubDirectories { get; set; } 
		public string Name { get; set; }
	}

	public enum Action
	{
		Command,
		Directory,
		File
	}

	public enum Direction
	{
		Up,
		Down,
		Stay
	}
}
