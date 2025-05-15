#!/usr/bin/env python3

"""

The below code is generate by Claude 3.7 Sonnet
"""

"""
V8 Codebase Context Builder

This script walks through a V8 codebase, analyzes each C++ file using the Gemini 2.0 Flash model,
and extracts information about functions, classes, imports, and their logic.
The analysis is logged to individual files with metadata including file paths.
"""

import os
import argparse
import json
import logging
import time
from pathlib import Path
import google.generativeai as genai
from typing import Dict, List, Any, Optional
import concurrent.futures
import random
import xml.etree.ElementTree as ET
import re

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler(f"v8_context_builder_{time.time()}.log"),
        logging.StreamHandler()
    ]
)

# Gemini API configuration
GEMINI_MODEL = "gemini-2.0-flash"

def setup_gemini_api(api_key: str) -> None:
    """Configure the Gemini API with the provided key."""
    genai.configure(api_key=api_key)
    logging.info(f"Configured Gemini API with model: {GEMINI_MODEL}")

def _parse_code_block_xml_element(element: ET.Element, file_path_for_logging: str) -> Dict[str, Any]:
    """
    Parses an XML element representing a code block (e.g., func, class, imports)
    into a dictionary with 'metadata' and 'code'.
    """
    data = {}
    metadata_el = element.find('metadata')
    if metadata_el is not None and metadata_el.text:
        try:
            # Ensure metadata text is stripped before parsing
            data['metadata'] = json.loads(metadata_el.text.strip())
        except json.JSONDecodeError as e:
            logging.warning(f"File {file_path_for_logging}: Failed to parse JSON in <metadata> of <{element.tag}>: {e}. Raw text: '{metadata_el.text.strip()[:100]}...'")
            data['metadata'] = {"error": "JSON parsing error in metadata", "raw_text": metadata_el.text.strip()}
    else:
        data['metadata'] = {}
        if metadata_el is None:
            logging.debug(f"File {file_path_for_logging}: No <metadata> tag found in <{element.tag}>.")
        elif metadata_el.text is None or not metadata_el.text.strip():
            logging.debug(f"File {file_path_for_logging}: Empty <metadata> tag in <{element.tag}>.")

    code_el = element.find('code')
    if code_el is not None and code_el.text:
        data['code'] = code_el.text.strip()
    else:
        data['code'] = ""
        if code_el is None:
            logging.debug(f"File {file_path_for_logging}: No <code> tag found in <{element.tag}>.")
        elif code_el.text is None or not code_el.text.strip():
             logging.debug(f"File {file_path_for_logging}: Empty <code> tag in <{element.tag}>.")
    return data

def _parse_file_xml_element(file_element: ET.Element, file_path_for_logging: str) -> Dict[str, Any]:
    """
    Parses the root <file> XML element into a structured dictionary.
    """
    result = {}
    if file_element.tag != 'file':
        logging.warning(f"File {file_path_for_logging}: Expected root XML tag <file>, got <{file_element.tag}>. Attempting to process children directly.")
    
    for child in file_element:
        if child.tag == 'metadata':
            if child.text:
                try:
                    # Ensure metadata text is stripped before parsing
                    result['metadata'] = json.loads(child.text.strip())
                except json.JSONDecodeError as e:
                    logging.warning(f"File {file_path_for_logging}: Failed to parse JSON in <file><metadata>: {e}. Raw text: '{child.text.strip()[:100]}...'")
                    result['metadata'] = {"error": "JSON parsing error in file metadata", "raw_text": child.text.strip()}
            else:
                result['metadata'] = {}
                logging.debug(f"File {file_path_for_logging}: Empty <file><metadata> tag.")
        elif child.tag == 'dependencies':
            deps_list = []
            for dep_item_el in child: 
                item_dict = _parse_code_block_xml_element(dep_item_el, file_path_for_logging)
                item_dict['type'] = dep_item_el.tag 
                deps_list.append(item_dict)
            result['dependencies'] = deps_list
        elif child.tag == 'imports': 
            result['imports'] = _parse_code_block_xml_element(child, file_path_for_logging)
        elif child.tag == 'func':
            result.setdefault('functions', []).append(_parse_code_block_xml_element(child, file_path_for_logging))
        elif child.tag == 'class':
            result.setdefault('classes', []).append(_parse_code_block_xml_element(child, file_path_for_logging))
        elif child.tag == 'interface':
            result.setdefault('interfaces', []).append(_parse_code_block_xml_element(child, file_path_for_logging))
        elif child.tag == 'code': # Handle top-level code block not fitting other categories
            if child.text:
                result['file_level_code_content'] = child.text.strip()
            else:
                result['file_level_code_content'] = ""
            logging.debug(f"File {file_path_for_logging}: Found and parsed top-level <code> block under <file>.")
        else:
            logging.warning(f"File {file_path_for_logging}: Unexpected tag <{child.tag}> in <file> element. Skipping.")
    return result

def build_context(file_path: str, model) -> Optional[Dict[str, Any]]:
    """
    Analyze a single C++ file using Gemini model to extract functions, classes, imports,
    and their logic descriptions.
    
    Args:
        file_path: Path to the C++ file
        model: Configured Gemini model
        
    Returns:
        Dictionary containing the analysis results, or None if skipped.
    """
    try:
        with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
            file_content = f.read()
        
        if len(file_content) == 0:
            logging.warning(f"Skipping empty file: {file_path}")
            return None
        
        if len(file_content) > 100000:
            logging.warning(f"File {file_path} is large ({len(file_content)} chars). Truncating...")
            file_content = file_content[:100000] + "\n// ... [truncated due to size]"
        
        prompt = f"""
                # Role and Purpose
                You are an AI assistant specialized in programming and software engineering. Your primary responsibility is to build comprehensive context for an LLM that performs code migration between programming languages. You will analyze source files, categorize code elements, and create structured representations with metadata.

                # File Analysis Structure
                For each file, create a structured representation using the following format:

                <file>
                    <metadata>
                    {{
                        "path": "path/to/file.ext",
                        "file_name": "file.ext",
                        "language": "language_name",
                        "purpose": "Brief description of the file's purpose and functionality"
                    }}
                    </metadata>
                    <dependencies>
                        // List dependent code elements that are referenced but not defined in this file
                        // Include minimal structural information for external dependencies
                    </dependencies>
                    
                    // File content blocks go here...
                </file>

                # Code Block Types and Structure
                Categorize code into the following block types:

                ## Import Blocks
                <imports>
                    <metadata>
                    {{
                        "language": "language_name",
                        "purpose": "What functionality these imports provide"
                    }}
                    </metadata>
                    <code>
                        // Import statements go here
                    </code>
                </imports>

                ## Function Blocks
                <func>
                    <metadata>
                    {{
                        "language": "language_name",
                        "type": "function",
                        "name": "functionName",
                        "parent": "ParentClass",
                        "about": "Brief description of what the function does",
                        "logic": "Detailed explanation of algorithm or implementation logic",
                        "parameters": [
                            {{
                                "name": "paramName",
                                "type": "paramType",
                                "purpose": "What this parameter is used for"
                            }}
                        ],
                        "return": {{
                            "type": "returnType",
                            "description": "What is returned"
                        }},
                        "dependencies": [
                            "DependencyName"
                        ]
                    }}
                    </metadata>
                    <code>
                        // Function code goes here
                    </code>
                </func>

                ## Class Blocks
                <class>
                    <metadata>
                    {{
                        "language": "language_name",
                        "type": "class",
                        "name": "ClassName",
                        "extends": "ParentClass",
                        "implements": [
                            "InterfaceName"
                        ],
                        "about": "Brief description of the class purpose",
                        "attributes": [
                            {{
                                "name": "attributeName",
                                "type": "attributeType",
                                "access": "public/private/protected",
                                "purpose": "What this attribute represents"
                            }}
                        ],
                        "dependencies": [
                            "DependencyName"
                        ]
                    }}
                    </metadata>
                    <code>
                        // Class code goes here
                    </code>
                </class>

                ## Interface Blocks
                <interface>
                    <metadata>
                    {{
                        "language": "language_name",
                        "type": "interface",
                        "name": "InterfaceName",
                        "extends": [
                            "ParentInterface"
                        ],
                        "about": "Brief description of the interface purpose",
                        "methods": [
                            {{
                                "name": "methodName",
                                "parameters": [
                                    {{
                                        "name": "paramName",
                                        "type": "paramType"
                                    }}
                                ],
                                "return": "returnType",
                                "purpose": "Method purpose"
                            }}
                        ],
                        "dependencies": [
                            "DependencyName"
                        ]
                    }}
                    </metadata>
                    <code>
                        // Interface code goes here
                    </code>
                </interface>

                # Analysis Guidelines
                1. Identify each logical code unit and place it in the appropriate block type
                2. Provide detailed metadata that explains the purpose and behavior
                3. Document dependencies between components to facilitate migration
                4. Include information about language-specific features that might require special handling
                5. For complex algorithms, explain the implementation logic clearly
                6. Note any potential migration challenges for specific code constructs

                # Example Output
                <file>
                    <metadata>
                    {{
                        "path": "src/compiler/frame.cpp",
                        "file_name": "frame.cpp",
                        "language": "cpp",
                        "purpose": "Implements stack frame management for the compiler"
                    }}
                    </metadata>
                    <dependencies>
                        <class>
                            <metadata>
                            {{
                                "language": "cpp",
                                "type": "class",
                                "name": "AlignedSlotAllocator",
                                "about": "Manages aligned memory allocation in slots"
                            }}
                            </metadata>
                            <code>
                                class AlignedSlotAllocator {{
                                public:
                                    static int NumSlotsForWidth(int width);
                                    void Align(int slot_count);
                                    int Size() const;
                                }};
                            </code>
                        </class>
                    </dependencies>

                    <func>
                        <metadata>
                        {{
                            "language": "cpp",
                            "type": "method",
                            "name": "AlignFrame",
                            "parent": "Frame",
                            "about": "Aligns the stack frame to the specified memory boundary",
                            "logic": "Ensures both return slots and regular slots are aligned to the power-of-2 boundary by adding padding slots as necessary",
                            "parameters": [
                                {{
                                    "name": "alignment",
                                    "type": "int",
                                    "purpose": "The memory alignment boundary (must be a power of 2)"
                                }}
                            ],
                            "return": {{
                                "type": "void",
                                "description": "No return value"
                            }},
                            "dependencies": [
                                "AlignedSlotAllocator",
                                "base::bits"
                            ]
                        }}
                        </metadata>
                        <code>
                            void Frame::AlignFrame(int alignment) {{
                            #if DEBUG
                            spill_slots_finished_ = true;
                            frame_aligned_ = true;
                            #endif
                            // In the calculations below we assume that alignment is a power of 2.
                            DCHECK(base::bits::IsPowerOfTwo(alignment));
                            int alignment_in_slots = AlignedSlotAllocator::NumSlotsForWidth(alignment);

                            // We have to align return slots separately, because they are claimed
                            // separately on the stack.
                            const int mask = alignment_in_slots - 1;
                            int return_delta = alignment_in_slots - (return_slot_count_ & mask);
                            if (return_delta != alignment_in_slots) {{
                                return_slot_count_ += return_delta;
                            }}
                            int delta = alignment_in_slots - (slot_allocator_.Size() & mask);
                            if (delta != alignment_in_slots) {{
                                slot_allocator_.Align(alignment_in_slots);
                                if (spill_slot_count_ != 0) {{
                                spill_slot_count_ += delta;
                                }}
                            }}
                            }}
                        </code>
                    </func>
                </file>

        Analyze this C++ file from the V8 JavaScript engine codebase:
        
        Path: {file_path}
        
        Code content:
        ```cpp
        {file_content}
        ```
        """

        delay = random.uniform(25, 40)
        logging.info(f"Sleeping for {delay:.2f} seconds before processing {file_path}")
        time.sleep(delay)

        response = model.generate_content(prompt)
        result_text = response.text
        
        parsed_result = None
        json_decode_error_str = None

        # Attempt 1: Parse as JSON
        json_start = result_text.find('{')
        json_end = result_text.rfind('}') + 1
        
        if json_start != -1 and json_end > json_start:
            json_text_candidate = result_text[json_start:json_end]
            try:
                # A simple check: if it looks like XML, don't even try JSON
                if not ("<" in json_text_candidate and ">" in json_text_candidate and ("<file>" in result_text or "<metadata>" in result_text)):
                    parsed_result = json.loads(json_text_candidate)
                    logging.info(f"Successfully parsed response as JSON for {file_path}")
                else:
                    logging.warning(f"Response for {file_path} contains JSON-like delimiters but also XML tags; preferring XML parsing. Snippet: {json_text_candidate[:100]}")
                    json_decode_error_str = "Skipped JSON parsing due to presence of XML tags."
            except json.JSONDecodeError as e:
                json_decode_error_str = str(e)
                logging.warning(f"Failed to parse extracted text as JSON for {file_path}: {e}. Extracted text: '{json_text_candidate[:200]}...' Attempting XML parsing.")
        else:
            logging.warning(f"No top-level JSON object {{...}} found in response for {file_path}. Attempting XML parsing.")

        # Attempt 2: Parse as XML if JSON parsing failed or was not applicable
        if parsed_result is None:
            logging.info(f"Attempting XML parsing for {file_path}.")
            # Attempt to strip markdown code fences
            text_to_parse_xml = result_text.strip()
            # Regex to match ```optional_lang\nCONTENT\n```
            # MODIFIED REGEX HERE:
            fence_match = re.match(r"^```(?:[a-zA-Z0-9_.-]*)?\s*(?:\n)?([\s\S]*?)\s*(?:\n)?```$", text_to_parse_xml, re.DOTALL)
            if fence_match:
                text_to_parse_xml = fence_match.group(1).strip()
                logging.info(f"Stripped markdown code fences for XML parsing for {file_path}")
            else:
                # Check if the whole response is wrapped in <file>...</file> without fences
                if not text_to_parse_xml.startswith("<file>"):
                    # Try to find <file> tag somewhere in the text if not at the beginning
                    file_tag_start = text_to_parse_xml.find("<file>")
                    if file_tag_start != -1:
                        # Potentially extract from <file> to its corresponding </file>
                        # This is complex; for now, assume if <file> is not at start (post-fence-strip), it's problematic
                        logging.warning(f"XML content for {file_path} does not start with <file> tag after initial stripping. Trying to parse as is but might fail. Content starts with: {text_to_parse_xml[:100]}")
            
            # Sanitize control characters that are invalid in XML 1.0
            sanitized_xml_text = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', text_to_parse_xml)
            
            if not sanitized_xml_text.strip().startswith("<"):
                logging.error(f"Response for {file_path} (after potential fence stripping) is not JSON and does not start with '<', unlikely to be XML. Raw response snippet: {sanitized_xml_text[:200]}")
                return {"file_path": file_path, "error": "Response not JSON and not XML-like after cleanup", 
                        "json_error_if_any": json_decode_error_str, "raw_response": result_text}
            try:
                root_element = ET.fromstring(sanitized_xml_text)
                if root_element.tag == 'file':
                    parsed_result = _parse_file_xml_element(root_element, file_path)
                    logging.info(f"Successfully parsed response as XML for {file_path}")
                else:
                    logging.error(f"XML root tag is <{root_element.tag}>, expected <file> for {file_path}. Raw response snippet: {result_text[:500]}")
                    return {"file_path": file_path, "error": f"XML root tag not <file> (was <{root_element.tag}>)", 
                            "json_error_if_any": json_decode_error_str, "raw_response": result_text}
            except ET.ParseError as xml_e:
                logging.error(f"Failed to parse response as XML for {file_path}: {xml_e}. Sanitized XML tried: {sanitized_xml_text[:500]}")
                return {"file_path": file_path, "error": "JSON and XML parsing failed", 
                        "json_error_if_any": json_decode_error_str, "xml_error": str(xml_e), "raw_response": result_text}
            except Exception as general_xml_e: 
                logging.error(f"An unexpected error occurred during XML processing for {file_path}: {general_xml_e}")
                return {"file_path": file_path, "error": "Unexpected error during XML processing", 
                        "json_error_if_any": json_decode_error_str, "xml_processing_error": str(general_xml_e), "raw_response": result_text}

        if parsed_result:
            # Ensure file_path from the argument is present at the top level of the result
            # This overwrites any path that might have come from the LLM's <metadata>
            # to ensure consistency with what the script expects.
            if isinstance(parsed_result, dict):
                 parsed_result["file_path"] = file_path
            else: # Should not happen if parsing is successful
                 logging.error(f"Parsed result for {file_path} is not a dictionary. Type: {type(parsed_result)}")
                 return {"file_path": file_path, "error": "Parsed result not a dictionary", "raw_response": result_text}
            return parsed_result
        else:
            logging.error(f"Failed to parse response for {file_path} using any method.")
            return {"file_path": file_path, "error": "No parseable content (JSON or XML)", 
                    "json_error_if_any": json_decode_error_str, "raw_response": result_text}
            
    except genai.types.generation_types.BlockedPromptException as bpe:
        logging.error(f"Prompt blocked for file {file_path}: {bpe}")
        return {"file_path": file_path, "error": "Prompt blocked by API", "details": str(bpe)}
    except genai.types.generation_types.StopCandidateException as sce:
        logging.error(f"Generation stopped unexpectedly for file {file_path}: {sce}")
        return {"file_path": file_path, "error": "Generation stopped by API", "details": str(sce)}
    except Exception as e:
        logging.error(f"Generic error analyzing file {file_path}: {e.__class__.__name__} - {e}")
        return {"file_path": file_path, "error": str(e)}


def save_analysis(analysis_result: Dict[str, Any], output_dir: str, file_path_orig: str) -> None:
    """
    Save the analysis result to a JSON file in the output directory.
    
    Args:
        analysis_result: The analysis result dictionary
        output_dir: Directory to save the output file
        file_path_orig: The original file path, used for constructing output path
    """
    if not analysis_result:
        return
    
    path_to_use = file_path_orig
    path_to_use = path_to_use.lstrip('/').lstrip('\\')
    if ':' in path_to_use: 
        path_to_use = path_to_use.split(':', 1)[1].lstrip('/').lstrip('\\')
    
    relative_dir = os.path.dirname(path_to_use)
    output_subdir = os.path.join(output_dir, relative_dir)
    os.makedirs(output_subdir, exist_ok=True)
    
    file_name_base = os.path.basename(path_to_use)
    output_file = os.path.join(output_subdir, f"{file_name_base}.context")
    
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(analysis_result, f, indent=2)
    
    logging.info(f"Saved analysis for {file_path_orig} to {output_file}")


def find_cpp_files(source_dir: str) -> List[str]:
    """
    Find all C++ source files in the given directory.
    
    Args:
        source_dir: Directory to search for C++ files
        
    Returns:
        List of file paths
    """
    cpp_extensions = ['.cc', '.cpp', '.cxx', '.h', '.hpp', '.c']
    cpp_files = []
    
    for root, _, files in os.walk(source_dir):
        for file in files:
            if any(file.endswith(ext) for ext in cpp_extensions):
                cpp_files.append(os.path.join(root, file))
    
    logging.info(f"Found {len(cpp_files)} C++ files in {source_dir}")
    return cpp_files

def build_codebase_context(source_dir: str, output_dir: str, api_key: str, max_workers: int = 4, 
                        limit: Optional[int] = None) -> None:
    """
    Main function to build context codebase.
    
    Args:
        source_dir: Directory containing the V8 source code
        output_dir: Directory to save analysis results
        api_key: Gemini API key
        max_workers: Maximum number of concurrent workers
        limit: Optional limit on the number of files to process
    """
    os.makedirs(output_dir, exist_ok=True)
    
    setup_gemini_api(api_key)
    # Add safety settings to the model generation config if needed
    # For example, to be less restrictive if that's an issue:
    # safety_settings = [
    #     {"category": "HARM_CATEGORY_HARASSMENT", "threshold": "BLOCK_NONE"},
    #     {"category": "HARM_CATEGORY_HATE_SPEECH", "threshold": "BLOCK_NONE"},
    #     {"category": "HARM_CATEGORY_SEXUALLY_EXPLICIT", "threshold": "BLOCK_NONE"},
    #     {"category": "HARM_CATEGORY_DANGEROUS_CONTENT", "threshold": "BLOCK_NONE"},
    # ]
    # model = genai.GenerativeModel(GEMINI_MODEL, safety_settings=safety_settings)
    model = genai.GenerativeModel(GEMINI_MODEL)

    cpp_files = find_cpp_files(source_dir)
    
    if limit:
        cpp_files = cpp_files[:limit]
        logging.info(f"Limited processing to {limit} files")
    
    processed_count = 0
    error_count = 0
    skipped_empty_count = 0
    
    with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
        futures = {executor.submit(build_context, file_path, model): file_path for file_path in cpp_files}
        
        for i, future in enumerate(concurrent.futures.as_completed(futures)):
            file_path_key = futures[future]
            try:
                result = future.result()
                if result:
                    if "error" not in result:
                        save_analysis(result, output_dir, file_path_key)
                        processed_count += 1
                    else:
                        logging.error(f"Analysis for {file_path_key} resulted in an error: {result.get('error')}. Details: {result.get('details', '')}. Raw response snippet: {result.get('raw_response', '')[:200]}")
                        # Optionally save full error reports
                        error_output_dir = os.path.join(output_dir, "errors")
                        os.makedirs(error_output_dir, exist_ok=True)
                        
                        # Construct error file name based on relative path
                        relative_path = os.path.relpath(file_path_key, source_dir)
                        # Replace path separators with underscores for a flat error file structure
                        sanitized_relative_path = relative_path.replace(os.sep, '_')
                        error_file_name = f"{sanitized_relative_path}.error.json"
                        
                        error_file_path = os.path.join(error_output_dir, error_file_name)
                        with open(error_file_path, 'w', encoding='utf-8') as ef:
                           json.dump(result, ef, indent=2)
                        logging.info(f"Saved error report for {file_path_key} to {error_file_path}")
                        error_count += 1
                else: 
                    logging.warning(f"No result for {file_path_key} (e.g. skipped empty file by build_context).")
                    skipped_empty_count +=1
            except Exception as e:
                logging.error(f"Error retrieving result future for {file_path_key}: {e.__class__.__name__} - {e}")
                error_count += 1
            
            total_attempted = i + 1
            if total_attempted % 10 == 0 or total_attempted == len(cpp_files):
                logging.info(f"Progress: {total_attempted}/{len(cpp_files)} files attempted (Successful: {processed_count}, Errors: {error_count}, Skipped Empty: {skipped_empty_count})")
    
    logging.info(f"Analysis complete. Total files: {len(cpp_files)}. Successfully processed: {processed_count}. Errors: {error_count}. Skipped (empty): {skipped_empty_count}")


def main():
    parser = argparse.ArgumentParser(description="Analyze V8 JavaScript Engine Codebase")
    parser.add_argument("--source", required=True, help="Path to V8 source code directory")
    parser.add_argument("--output", required=True, help="Path to output directory for analysis")
    parser.add_argument("--api-key", required=True, help="Gemini API key")
    parser.add_argument("--workers", type=int, default=4, help="Maximum number of concurrent workers")
    parser.add_argument("--limit", type=int, help="Limit the number of files to process (for testing)")
    
    args = parser.parse_args()
    
    logging.info(f"Starting V8 codebase analysis with source: {args.source}, output: {args.output}")
    start_time = time.time()
    
    build_codebase_context(
        source_dir=args.source,
        output_dir=args.output,
        api_key=args.api_key,
        max_workers=args.workers,
        limit=args.limit
    )
    
    elapsed_time = time.time() - start_time
    logging.info(f"Analysis completed in {elapsed_time:.2f} seconds")

if __name__ == "__main__":
    main()