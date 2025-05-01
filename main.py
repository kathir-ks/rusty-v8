import os
import fnmatch
import argparse
from pathlib import Path
from typing import List, Optional, Set

# --- Configuration ---

# Common directories/files to exclude by default
DEFAULT_EXCLUDE_PATTERNS = [
    ".git/",
    ".svn/",
    "__pycache__/",
    "*.pyc",
    "*.pyo",
    "*.so",
    "*.o",
    "*.a",
    "*.dll",
    "*.exe",
    "*.dylib",
    "*.egg-info/",
    "dist/",
    "build/",
    "node_modules/",
    "vendor/",
    "target/",  # Rust build output
    ".DS_Store",
    "*.log",
    "*.tmp",
    "*.swp",
    "*.swo",
    # Add more common patterns as needed
]

# Common binary file extensions (heuristic)
BINARY_EXTENSIONS = {
    '.png', '.jpg', '.jpeg', '.gif', '.bmp', '.ico', '.tif', '.tiff',
    '.pdf', '.doc', '.docx', '.xls', '.xlsx', '.ppt', '.pptx',
    '.zip', '.tar', '.gz', '.bz2', '.rar', '.7z',
    '.mp3', '.wav', '.ogg', '.flac',
    '.mp4', '.avi', '.mov', '.wmv', '.mkv',
    '.sqlite', '.db',
    '.jar', '.war', '.ear',
    # Add more as needed
}

# --- Core Logic ---

def is_likely_text_file(file_path: Path) -> bool:
    """
    Checks if a file is likely a text file based on extension and content sniffing.
    """
    # 1. Check extension
    if file_path.suffix.lower() in BINARY_EXTENSIONS:
        return False

    # 2. Try reading a small chunk as UTF-8 (common for code)
    try:
        with open(file_path, 'rb') as f:
            chunk = f.read(1024)  # Read first 1KB
            # Check for null bytes, common in binary files
            if b'\x00' in chunk:
                return False
            # Try decoding as UTF-8
            chunk.decode('utf-8')
            return True
    except UnicodeDecodeError:
        # Failed to decode as UTF-8, likely not text or different encoding
        return False
    except IOError:
        # Couldn't read the file
        return False
    except Exception:
        # Other potential issues
        return False

def find_code_files(
    paths: List[str],
    include_patterns: Optional[List[str]] = None,
    exclude_patterns: Optional[List[str]] = None,
    use_default_excludes: bool = True,
    verbose: bool = False,
    max_file_size_kb: Optional[int] = None,
) -> List[Path]:
    """
    Finds relevant code files in the given paths, applying inclusion/exclusion rules.
    """
    base_paths: List[Path] = [Path(p).resolve() for p in paths]
    found_files: Set[Path] = set()
    processed_paths: Set[Path] = set() # To avoid processing the same file/dir multiple times

    # Combine default and custom exclude patterns
    all_exclude_patterns = list(exclude_patterns) if exclude_patterns else []
    if use_default_excludes:
        all_exclude_patterns.extend(DEFAULT_EXCLUDE_PATTERNS)

    for base_path in base_paths:
        if base_path in processed_paths:
            continue

        if base_path.is_file():
            # If a specific file is provided
            if base_path not in processed_paths:
                absolute_path_str = str(base_path)
                # Check excludes first
                is_excluded = any(fnmatch.fnmatch(absolute_path_str, pattern) or
                                  fnmatch.fnmatch(base_path.name, pattern) for pattern in all_exclude_patterns)

                if is_excluded:
                    if verbose: print(f"Excluding (explicit path, matched exclude): {base_path}")
                    processed_paths.add(base_path)
                    continue

                # Check includes if provided
                is_included = True
                if include_patterns:
                    is_included = any(fnmatch.fnmatch(absolute_path_str, pattern) or
                                      fnmatch.fnmatch(base_path.name, pattern) for pattern in include_patterns)

                if not is_included:
                     if verbose: print(f"Skipping (explicit path, didn't match include): {base_path}")
                     processed_paths.add(base_path)
                     continue

                found_files.add(base_path)
                processed_paths.add(base_path)
                if verbose: print(f"Adding explicit file: {base_path}")


        elif base_path.is_dir():
             # If a directory is provided, walk through it
            if verbose: print(f"Scanning directory: {base_path}")
            for root, dirs, files in os.walk(base_path, topdown=True):
                root_path = Path(root).resolve()

                # --- Directory Exclusion ---
                # Modify dirs in-place to prevent recursing into excluded directories
                excluded_dirs = []
                for d in dirs:
                    dir_path = root_path / d
                    dir_path_str = str(dir_path) + "/" # Add trailing slash for dir matching
                    if dir_path in processed_paths:
                        excluded_dirs.append(d)
                        continue

                    is_excluded = any(fnmatch.fnmatch(dir_path_str, pattern) or
                                      fnmatch.fnmatch(d + "/", pattern) or
                                      fnmatch.fnmatch(d, pattern.rstrip('/')) # Match dir name too
                                      for pattern in all_exclude_patterns)

                    if is_excluded:
                        if verbose: print(f"  Excluding directory: {dir_path}")
                        excluded_dirs.append(d)
                        processed_paths.add(dir_path) # Mark dir as processed to avoid re-check

                # Remove excluded directories from the list os.walk will visit
                dirs[:] = [d for d in dirs if d not in excluded_dirs]


                # --- File Processing ---
                for filename in files:
                    file_path = root_path / filename
                    if file_path in processed_paths:
                        continue

                    absolute_path_str = str(file_path)
                    processed_paths.add(file_path) # Mark file as processed

                    # 1. Check Excludes
                    is_excluded = any(fnmatch.fnmatch(absolute_path_str, pattern) or
                                      fnmatch.fnmatch(filename, pattern) for pattern in all_exclude_patterns)
                    if is_excluded:
                        if verbose: print(f"  Excluding file (matched exclude): {file_path}")
                        continue

                    # 2. Check Includes (if specified)
                    if include_patterns:
                        is_included = any(fnmatch.fnmatch(absolute_path_str, pattern) or
                                          fnmatch.fnmatch(filename, pattern) for pattern in include_patterns)
                        if not is_included:
                            if verbose: print(f"  Skipping file (didn't match include): {file_path}")
                            continue

                    # 3. Check File Size
                    if max_file_size_kb is not None:
                        try:
                            file_size_kb = file_path.stat().st_size / 1024
                            if file_size_kb > max_file_size_kb:
                                if verbose: print(f"  Skipping file (too large: {file_size_kb:.2f} KB): {file_path}")
                                continue
                        except OSError as e:
                             if verbose: print(f"  Warning: Could not get size for {file_path}: {e}")
                             continue # Skip if cannot get size

                    # 4. Check if likely text file
                    if not is_likely_text_file(file_path):
                         if verbose: print(f"  Skipping file (likely binary): {file_path}")
                         continue

                    # If all checks pass, add the file
                    found_files.add(file_path)
                    if verbose: print(f"  Adding file: {file_path}")
            processed_paths.add(base_path) # Mark base directory as processed

        else:
            print(f"Warning: Path '{base_path}' is neither a file nor a directory. Skipping.")
            processed_paths.add(base_path)


    return sorted(list(found_files)) # Return sorted list for consistent order

def create_llm_prompt(
    base_prompt: str,
    code_files: List[Path],
    base_dir: Optional[Path] = None,
    verbose: bool = False
) -> str:
    """
    Creates the final LLM prompt string with formatted code content.
    """
    prompt_parts = [base_prompt]
    prompt_parts.append("\n\n--- Start of Codebase Content ---")

    if not code_files:
        prompt_parts.append("\n\n(No code files were included based on the specified criteria.)")
    else:
        # Try to find a common base directory for cleaner relative paths
        if base_dir is None:
            try:
                base_dir = Path(os.path.commonpath([str(p) for p in code_files if p.is_file()]))
                # If common path is a file's parent, use that. Otherwise, might need parent of common dir.
                if not base_dir.is_dir():
                    base_dir = base_dir.parent
            except ValueError: # Happens if paths are on different drives (Windows) or empty list
                 base_dir = None # Fallback to absolute paths

        if verbose and base_dir:
            print(f"Using common base directory for relative paths: {base_dir}")

        for file_path in code_files:
            try:
                # Determine relative path if possible
                if base_dir and base_dir in file_path.parents:
                     display_path = file_path.relative_to(base_dir)
                else:
                    display_path = file_path # Fallback to absolute path

                prompt_parts.append(f"\n\n--- File: {display_path} ---")
                # Try reading with UTF-8, fallback to latin-1 if needed (less ideal but better than crashing)
                try:
                    content = file_path.read_text(encoding='utf-8')
                except UnicodeDecodeError:
                    if verbose: print(f"Warning: Could not decode {file_path} as UTF-8, trying latin-1.")
                    try:
                         content = file_path.read_text(encoding='latin-1')
                    except Exception as e_inner:
                         if verbose: print(f"Error reading {file_path} with fallback encoding: {e_inner}")
                         content = f"(Error reading file content: {e_inner})"
                except Exception as e:
                    if verbose: print(f"Error reading {file_path}: {e}")
                    content = f"(Error reading file content: {e})"

                # Basic language detection for markdown code block
                lang = file_path.suffix.lstrip('.').lower()
                if not lang: lang = "text" # Default language if no extension

                prompt_parts.append(f"```{lang}\n{content}\n```")

            except Exception as e:
                prompt_parts.append(f"\n\n--- Error processing file: {file_path} ---")
                prompt_parts.append(f"(Could not read or format file: {e})")
                if verbose: print(f"Error processing {file_path}: {e}")


    prompt_parts.append("\n\n--- End of Codebase Content ---")
    prompt_parts.append("\n\nPlease analyze the provided codebase content to answer the request.")

    return "".join(prompt_parts)

# --- Command Line Interface ---

def main():
    parser = argparse.ArgumentParser(
        description="Generate an LLM prompt including codebase content.",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter
    )
    parser.add_argument(
        "base_prompt",
        help="The base instruction or question for the LLM."
    )
    parser.add_argument(
        "paths",
        nargs='+',
        help="One or more file or directory paths to include in the prompt."
    )
    parser.add_argument(
        "-o", "--output",
        help="Optional path to save the generated prompt to a file."
    )
    parser.add_argument(
        "-i", "--include",
        action='append',
        dest='include_patterns',
        help="Glob pattern for files/paths to explicitly include (can be used multiple times). "
             "If specified, only files matching these patterns are considered (after excludes)."
    )
    parser.add_argument(
        "-x", "--exclude",
        action='append',
        dest='exclude_patterns',
        help=f"Glob pattern for files/directories to exclude (can be used multiple times). "
             f"Overrides default excludes if --no-default-excludes is used."
    )
    parser.add_argument(
        "--no-default-excludes",
        action='store_false',
        dest='use_default_excludes',
        help="Do not use the default exclude patterns."
    )
    parser.add_argument(
        "--max-size",
        type=int,
        dest='max_file_size_kb',
        help="Maximum individual file size in KB to include."
    )
    parser.add_argument(
        "-v", "--verbose",
        action='store_true',
        help="Print verbose output about file discovery and processing."
    )

    args = parser.parse_args()

    if args.verbose:
        print("--- Configuration ---")
        print(f"Base Prompt: '{args.base_prompt[:50]}...'")
        print(f"Paths: {args.paths}")
        print(f"Output File: {args.output}")
        print(f"Include Patterns: {args.include_patterns}")
        print(f"Exclude Patterns: {args.exclude_patterns}")
        print(f"Use Default Excludes: {args.use_default_excludes}")
        print(f"Max File Size (KB): {args.max_file_size_kb}")
        print("---------------------\n")


    # 1. Find files
    code_files = find_code_files(
        paths=args.paths,
        include_patterns=args.include_patterns,
        exclude_patterns=args.exclude_patterns,
        use_default_excludes=args.use_default_excludes,
        verbose=args.verbose,
        max_file_size_kb=args.max_file_size_kb
    )

    if not code_files:
        print("Warning: No code files found matching the criteria.")
        # Decide if you want to proceed with an empty codebase prompt or exit
        # For this example, we'll proceed

    # 2. Create prompt
    # Determine a sensible base_dir for relative paths
    common_base_dir = None
    if args.paths:
         try:
             # Use only the input directories/file parents for common path calculation
             input_dirs_parents = []
             for p_str in args.paths:
                 p = Path(p_str).resolve()
                 if p.is_dir():
                     input_dirs_parents.append(str(p))
                 elif p.is_file():
                     input_dirs_parents.append(str(p.parent))

             if input_dirs_parents:
                 common_base_dir = Path(os.path.commonpath(input_dirs_parents))
         except ValueError:
             common_base_dir = None # Fallback if paths are disparate

    final_prompt = create_llm_prompt(
        base_prompt=args.base_prompt,
        code_files=code_files,
        base_dir=common_base_dir,
        verbose=args.verbose
    )

    # 3. Output prompt
    if args.output:
        try:
            output_path = Path(args.output)
            output_path.parent.mkdir(parents=True, exist_ok=True) # Ensure directory exists
            output_path.write_text(final_prompt, encoding='utf-8')
            print(f"\nPrompt successfully saved to: {output_path}")
        except Exception as e:
            print(f"\nError saving prompt to file {args.output}: {e}")
            print("\n--- Generated Prompt ---")
            print(final_prompt)
            print("--- End of Prompt ---")
    else:
        print("\n--- Generated Prompt ---")
        print(final_prompt)
        print("--- End of Prompt ---")

    # 4. Optional: Warn about prompt length (very basic estimate)
    prompt_len = len(final_prompt)
    print(f"\n--- Prompt Info ---")
    print(f"Generated prompt length: {prompt_len} characters.")
    print(f"Number of files included: {len(code_files)}")
    # A very rough token estimate (assuming ~4 chars/token)
    estimated_tokens = prompt_len / 4
    print(f"Estimated token count (very rough): {estimated_tokens:.0f}")
    print("Warning: LLMs have token limits. If this prompt is too long, it may be truncated or rejected.")
    print("Consider using more specific paths, include/exclude patterns, or --max-size.")
    print("---")


if __name__ == "__main__":
    main()