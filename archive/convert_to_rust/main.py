#!/usr/bin/env python3

"""
Enhanced V8 C++ to Rust Converter

This improved version addresses:
1. Header/source file pairing and merging
2. Cross-file dependency management
3. Better prompts to avoid placeholders
4. Shared context for consistent conversions
"""

import os
import argparse
import json
import logging
import time
from pathlib import Path
import google.generativeai as genai
from typing import Dict, List, Any, Optional, Tuple, Set
import concurrent.futures
import random
from collections import defaultdict
import hashlib

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler(f"v8_converter_enhanced_{time.time()}.log"),
        logging.StreamHandler()
    ]
)

# Gemini API configuration
GEMINI_MODEL = "gemini-2.0-flash"

class DependencyTracker:
    """Tracks dependencies and provides context for conversions."""
    
    def __init__(self):
        self.converted_files: Dict[str, str] = {}  # path -> rust_code
        self.file_signatures: Dict[str, str] = {}  # path -> signature hash
        self.dependencies: Dict[str, Set[str]] = defaultdict(set)  # file -> dependencies
        self.type_definitions: Dict[str, str] = {}  # type_name -> definition
        self.function_signatures: Dict[str, str] = {}  # function_name -> signature
    
    def add_converted_file(self, file_path: str, rust_code: str):
        """Add a converted file to the tracker."""
        self.converted_files[file_path] = rust_code
        self.file_signatures[file_path] = hashlib.md5(rust_code.encode()).hexdigest()
        self._extract_definitions(file_path, rust_code)
    
    def _extract_definitions(self, file_path: str, rust_code: str):
        """Extract type and function definitions from Rust code."""
        lines = rust_code.split('\n')
        for line in lines:
            line = line.strip()
            # Extract struct definitions
            if line.startswith('pub struct ') or line.startswith('struct '):
                parts = line.split()
                if len(parts) >= 2:
                    struct_name = parts[2].rstrip('{<(')
                    self.type_definitions[struct_name] = f"// From {file_path}\n{line}"
            
            # Extract enum definitions
            elif line.startswith('pub enum ') or line.startswith('enum '):
                parts = line.split()
                if len(parts) >= 2:
                    enum_name = parts[2].rstrip('{<(')
                    self.type_definitions[enum_name] = f"// From {file_path}\n{line}"
            
            # Extract function signatures
            elif line.startswith('pub fn ') or line.startswith('fn '):
                if '(' in line:
                    fn_name = line.split('(')[0].split()[-1]
                    self.function_signatures[fn_name] = f"// From {file_path}\n{line}"
    
    def get_relevant_context(self, file_path: str, content: str) -> str:
        """Get relevant context for converting a file."""
        context_parts = []
        
        # Add related type definitions
        words = content.split()
        for word in words:
            if word in self.type_definitions:
                context_parts.append(self.type_definitions[word])
        
        # Add related function signatures
        for word in words:
            if word in self.function_signatures:
                context_parts.append(self.function_signatures[word])
        
        return '\n'.join(context_parts) if context_parts else ""

class FileGroup:
    """Represents a group of related C++ files (header + implementation)."""
    
    def __init__(self, base_name: str):
        self.base_name = base_name
        self.header_file: Optional[str] = None
        self.impl_file: Optional[str] = None
        self.header_content: str = ""
        self.impl_content: str = ""
    
    def add_file(self, file_path: str, content: str):
        """Add a file to this group."""
        if file_path.endswith(('.h', '.hpp', '.hxx')):
            self.header_file = file_path
            self.header_content = content
        else:
            self.impl_file = file_path
            self.impl_content = content
    
    def is_complete(self) -> bool:
        """Check if we have both header and implementation."""
        return bool(self.header_file and self.impl_file)
    
    def get_combined_content(self) -> Tuple[str, str]:
        """Get combined content for conversion."""
        combined_content = ""
        file_info = ""
        
        if self.header_file:
            file_info += f"Header file: {self.header_file}\n"
            combined_content += f"// === HEADER CONTENT ===\n{self.header_content}\n\n"
        
        if self.impl_file:
            file_info += f"Implementation file: {self.impl_file}\n"
            combined_content += f"// === IMPLEMENTATION CONTENT ===\n{self.impl_content}\n"
        
        return file_info, combined_content

def setup_gemini_api(api_key: str) -> None:
    """Configure the Gemini API with the provided key."""
    genai.configure(api_key=api_key)
    logging.info(f"Configured Gemini API with model: {GEMINI_MODEL}")

def group_cpp_files(cpp_files: List[str]) -> Dict[str, FileGroup]:
    """Group C++ files by their base names."""
    groups = {}
    
    for file_path in cpp_files:
        # Get base name without extension
        base_name = os.path.splitext(os.path.basename(file_path))[0]
        
        # Create a more specific key that includes directory structure
        rel_dir = os.path.dirname(file_path)
        group_key = f"{rel_dir}/{base_name}"
        
        if group_key not in groups:
            groups[group_key] = FileGroup(base_name)
        
        # Read file content
        try:
            with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
                content = f.read()
            groups[group_key].add_file(file_path, content)
        except Exception as e:
            logging.error(f"Error reading file {file_path}: {e}")
    
    logging.info(groups)
    return groups


def create_enhanced_prompt(file_group: FileGroup, context: str, dependency_tracker: DependencyTracker) -> str:
    """Create an enhanced prompt for better Rust conversion."""
    
    file_info, combined_content = file_group.get_combined_content()
    
    # Get relevant context from previously converted files
    relevant_context = dependency_tracker.get_relevant_context(
        file_group.header_file or file_group.impl_file or "", 
        combined_content
    )
    
    prompt = f"""
You are an expert systems programmer converting V8 JavaScript engine C++ code to idiomatic Rust code.

CRITICAL REQUIREMENTS:
1. DO NOT use placeholder implementations like "// TODO" or "unimplemented!()"
2. Provide COMPLETE, WORKING implementations for all functions and methods
3. If you cannot determine the exact implementation, provide a reasonable working implementation based on the function signature and context
4. Use real Rust error handling with Result<T, E> types, not placeholder error handling

Files being converted:
{file_info}

{f"CONTEXT FROM PREVIOUSLY CONVERTED FILES:" if relevant_context else ""}
{relevant_context}

{f"ADDITIONAL CONTEXT:" if context else ""}
{context}

C++ CODE TO CONVERT:
```cpp
{combined_content}
```

CONVERSION GUIDELINES:

## Memory Management
- Convert raw pointers to appropriate Rust smart pointers (Box, Rc, Arc)
- Use lifetime parameters where needed
- Replace manual memory management with RAII patterns

## Error Handling
- Convert C++ exceptions to Result<T, E> types
- Use custom error enums for domain-specific errors
- Implement proper error propagation with ? operator

## Classes and Inheritance
- Convert C++ classes to Rust structs with impl blocks
- Use traits for interface inheritance
- Implement composition over inheritance patterns

## Templates and Generics
- Convert C++ templates to Rust generics with appropriate bounds
- Use associated types where appropriate
- Implement generic constraints properly

## Concurrency
- Use Rust's ownership system for thread safety
- Convert mutex usage to std::sync::Mutex or RwLock
- Use channels for inter-thread communication

## V8-Specific Patterns
- Convert V8 handles to appropriate Rust equivalents
- Transform V8 context handling to Rust patterns
- Adapt V8 object model to Rust's type system

## Implementation Requirements
- Every function MUST have a complete implementation
- Use realistic data structures and algorithms
- Provide working error paths, not just success paths
- Include proper initialization and cleanup code

## Code Quality
- Add comprehensive documentation comments
- Use idiomatic Rust naming conventions
- Include unit tests where appropriate
- Ensure all code compiles and follows Rust best practices

IMPORTANT: If the C++ code contains complex algorithms or data structures that you cannot fully understand from the context, implement a reasonable working version that maintains the same interface contract. Do not leave any function bodies empty or with placeholder implementations.

Output only the converted Rust code without markdown code blocks or explanations.
"""
    
    return prompt

def convert_file_group(file_group: FileGroup, model, context: str, dependency_tracker: DependencyTracker) -> Dict[str, Any]:
    """Convert a group of related C++ files to Rust."""
    max_retries = 3
    retry_count = 0
    
    # Validate file group has at least one file
    if not file_group.header_file and not file_group.impl_file:
        logging.warning(f"Skipping empty file group: {file_group.base_name}")
        return None
    
    file_info, combined_content = file_group.get_combined_content()
    
    # Skip if content is too large or empty
    if len(combined_content) == 0:
        logging.warning(f"Skipping empty file group: {file_group.base_name}")
        return None
    
    if len(combined_content) > 8000000:  # Reduce size limit for better quality
        logging.warning(f"File group {file_group.base_name} is large ({len(combined_content)} chars). Truncating...")
        combined_content = combined_content[:8000000] + "\n// ... [truncated due to size]"
    
    while retry_count < max_retries:
        try:
            prompt = create_enhanced_prompt(file_group, context, dependency_tracker)
            
            # Add random delay to prevent rate limiting
            # delay = random.uniform(45, 60)  # Increased delay for better quality
            delay = 300 # 5 minutes to limit the api requests to 250 RPD
            logging.info(f"Converting {file_group.base_name} (attempt {retry_count + 1})")
            time.sleep(delay)
            
            # Make API call
            response = model.generate_content(prompt)
            rust_code = response.text
            
            # Clean up response
            if "```rust" in rust_code:
                rust_blocks = []
                lines = rust_code.split('\n')
                in_code_block = False
                for line in lines:
                    if line.strip() in ["```rust", "```"]:
                        in_code_block = not in_code_block
                        continue
                    if in_code_block:
                        rust_blocks.append(line)
                rust_code = '\n'.join(rust_blocks)
            
            # Validate that we don't have placeholder implementations
            if "unimplemented!()" in rust_code or "// TODO" in rust_code:
                logging.warning(f"Generated code for {file_group.base_name} contains placeholders, retrying...")
                retry_count += 1
                continue
            
            # Add to dependency tracker - use whichever file is available
            file_path = file_group.header_file or file_group.impl_file
            if file_path:  # Additional safety check
                dependency_tracker.add_converted_file(file_path, rust_code)
            
            return {
                "base_name": file_group.base_name,
                "header_file": file_group.header_file,
                "impl_file": file_group.impl_file,
                "rust_code": rust_code,
                "success": True
            }
                
        except Exception as e:
            retry_count += 1
            if retry_count < max_retries:
                wait_time = 2 ** retry_count
                logging.warning(f"Attempt {retry_count} failed for {file_group.base_name}: {e}. Retrying in {wait_time} seconds...")
                time.sleep(wait_time)
            else:
                logging.error(f"All {max_retries} attempts failed for {file_group.base_name}: {e}")
                return {
                    "base_name": file_group.base_name,
                    "error": str(e),
                    "success": False
                }
    
    return {
        "base_name": file_group.base_name,
        "error": "Unknown error occurred after all retry attempts",
        "success": False
    }

def save_conversion(conversion_result: Dict[str, Any], source_dir: str, output_dir: str) -> None:
    """Save the converted Rust code."""
    if not conversion_result or not conversion_result.get("success", False):
        return
    
    base_name = conversion_result.get("base_name", "")
    rust_code = conversion_result.get("rust_code", "")
    
    # Determine output path based on the original file location
    original_file = conversion_result.get("header_file") or conversion_result.get("impl_file")
    if not original_file:
        logging.warning(f"No original file found for conversion result: {base_name}")
        return
    
    try:
        rel_path = os.path.relpath(original_file, source_dir)
        output_path = os.path.dirname(os.path.join(output_dir, rel_path))
        os.makedirs(output_path, exist_ok=True)
        
        # Create output filename - use just the base name part
        base_file_name = os.path.basename(base_name) if base_name else "unknown"
        output_file = os.path.join(output_path, f"{base_file_name}.rs")
        
        # Add module header
        header_name = os.path.basename(conversion_result.get('header_file', '')) if conversion_result.get('header_file') else 'N/A'
        impl_name = os.path.basename(conversion_result.get('impl_file', '')) if conversion_result.get('impl_file') else 'N/A'
        
        module_header = f"""// Converted from V8 C++ source files:
// Header: {header_name}
// Implementation: {impl_name}
// 
// This file combines both header and implementation into idiomatic Rust code.

{rust_code}
"""
        
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(module_header)
        
        logging.info(f"Saved Rust conversion to {output_file}")
        
    except Exception as e:
        logging.error(f"Error saving conversion for {base_name}: {e}")

def find_cpp_files(source_dir: str) -> List[str]:
    """Find all C++ source files."""
    cpp_extensions = ['.cc', '.cpp', '.cxx', '.h', '.hpp', '.c']
    cpp_files = []
    
    for root, _, files in os.walk(source_dir):
        for file in files:
            if any(file.endswith(ext) for ext in cpp_extensions):
                cpp_files.append(os.path.join(root, file))
    
    logging.info(f"Found {len(cpp_files)} C++ files in {source_dir}")
    return cpp_files

def convert_v8_codebase(source_dir: str, output_dir: str, api_key: str, max_workers: int = 2, 
                       limit: Optional[int] = None) -> None:
    """Enhanced main conversion function."""
    
    os.makedirs(output_dir, exist_ok=True)
    
    setup_gemini_api(api_key)
    model = genai.GenerativeModel(GEMINI_MODEL)
    
    dependency_tracker = DependencyTracker()
    
    # Find and group C++ files
    cpp_files = find_cpp_files(source_dir)
    file_groups = group_cpp_files(cpp_files)
    
    logging.info(f"Grouped {len(cpp_files)} files into {len(file_groups)} file groups")
    
    if limit:
        file_groups = dict(list(file_groups.items())[:limit])
        logging.info(f"Limited processing to {limit} file groups")
    
    # Convert file groups
    processed_count = 0
    error_count = 0
    
    # Process groups sequentially to build dependency context
    # Note: Using sequential processing to maintain dependency context
    # You could implement smarter batching based on dependency analysis
    
    for group_key, file_group in file_groups.items():
        try:
            # Provide context about current conversion state
            context = f"Currently processing file group {processed_count + 1}/{len(file_groups)}"
            
            result = convert_file_group(file_group, model, context, dependency_tracker)
            
            if result and result.get("success", False):
                save_conversion(result, source_dir, output_dir)
                processed_count += 1
            else:
                logging.warning(f"No conversion result for {group_key}")
                error_count += 1
                
        except Exception as e:
            logging.error(f"Error processing {group_key}: {e}")
            error_count += 1
        
        # Log progress
        total_processed = processed_count + error_count
        if total_processed % 5 == 0:
            logging.info(f"Progress: {total_processed}/{len(file_groups)} file groups processed")
    
    # Create Cargo.toml for the converted project
    cargo_toml_content = """[package]
name = "v8-rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# Add common dependencies that might be needed for V8 conversion
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
parking_lot = "0.12"
crossbeam = "0.8"
num-traits = "0.2"
bitflags = "2.0"

[dev-dependencies]
criterion = "0.5"
proptest = "1.0"

[[bench]]
name = "benchmarks"
harness = false
"""
    
    with open(os.path.join(output_dir, "Cargo.toml"), 'w') as f:
        f.write(cargo_toml_content)
    
    # Create lib.rs
    lib_rs_content = """//! V8 JavaScript Engine - Rust Port
//! 
//! This is an automated conversion of the V8 JavaScript engine from C++ to Rust.
//! The conversion maintains the original architecture while leveraging Rust's
//! memory safety and performance characteristics.

#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

// Re-export main modules
// Note: You'll need to add actual module declarations based on converted files

pub mod common;
pub mod base;
pub mod runtime;

// Common types and utilities
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
"""
    
    with open(os.path.join(output_dir, "lib.rs"), 'w') as f:
        f.write(lib_rs_content)
    
    logging.info(f"Conversion complete. Successfully converted {processed_count} file groups. Errors: {error_count}")
    logging.info(f"Created Rust project structure in {output_dir}")

def main():
    parser = argparse.ArgumentParser(description="Enhanced V8 C++ to Rust Converter")
    parser.add_argument("--source", required=True, help="Path to V8 C++ source code directory")
    parser.add_argument("--output", required=True, help="Path to output directory for Rust code")
    parser.add_argument("--api-key", required=True, help="Gemini API key")
    parser.add_argument("--workers", type=int, default=2, help="Maximum number of concurrent workers (reduced for better context)")
    parser.add_argument("--limit", type=int, help="Limit the number of file groups to process")
    
    args = parser.parse_args()
    
    logging.info("Starting enhanced V8 C++ to Rust conversion")
    start_time = time.time()
    
    convert_v8_codebase(
        source_dir=args.source,
        output_dir=args.output,
        api_key=args.api_key,
        max_workers=args.workers,
        limit=args.limit
    )
    
    elapsed_time = time.time() - start_time
    logging.info(f"Conversion completed in {elapsed_time:.2f} seconds")

if __name__ == "__main__":
    main()