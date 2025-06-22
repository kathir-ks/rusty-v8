import os
from pathlib import Path
from typing import Dict

# Only exclude .git directory
EXCLUDE_DIRS = {'.git'}

def count_lines_in_file(file_path: str) -> int:
    """Count the number of lines in a file, excluding empty lines."""
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            return sum(1 for line in file if line.strip())
    except (UnicodeDecodeError, IOError):
        # Skip files that can't be read as text
        return 0

def analyze_codebase(root_dir: str) -> Dict[str, int]:
    """
    Recursively analyze all files and count lines per file extension.
    Returns a dictionary with extension as key and total lines as value.
    """
    extension_counts: Dict[str, int] = {}
    total_files = 0

    for root, dirs, files in os.walk(root_dir):
        # Skip .git directory
        dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS]

        for file in files:
            file_path = Path(root) / file
            extension = file_path.suffix.lower()
            
            # Count lines for all files with extensions
            if extension:  # Only process files with extensions
                lines = count_lines_in_file(str(file_path))
                extension_counts[extension] = extension_counts.get(extension, 0) + lines
                total_files += 1

    return extension_counts, total_files

def main():
    # Get the current directory
    root_directory = os.getcwd()
    root_directory = '/home/kathirks_gc/v8'
    print(f"Analyzing all files in: {root_directory}")
    print("=" * 50)

    # Analyze the codebase
    extension_counts, total_files = analyze_codebase(root_directory)

    # Print results
    if extension_counts:
        print("\nLines by file type:")
        print("-" * 30)
        total_lines = 0
        stats = []
        for ext, count in sorted(extension_counts.items(), key=lambda x: x[1], reverse=True):
            print(f"{ext:<10} : {count:,} lines")
            stats.append(f'{ext:<10} : {count:,} lines\n')
            total_lines += count

        print("-" * 30)
        print(f"Total files: {total_files:,}")
        print(f"Total lines: {total_lines:,}")

        with open('stats.txt', 'w') as f:
            f.writelines(stats)

    else:
        print("No files found in the specified directory.")

if __name__ == "__main__":
    main() 