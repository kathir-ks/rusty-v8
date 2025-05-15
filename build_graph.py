import os
import json
import networkx as nx
import matplotlib.pyplot as plt # For basic visualization
# import pydot # For Graphviz via pydot, install if you want to use it
from networkx.drawing.nx_pydot import to_pydot # For Graphviz
import argparse
import re
from pathlib import Path

# --- Helper Functions ---

def get_node_id(file_path, item_type, item_name=None, parent_name=None):
    """Creates a unique node ID."""
    if item_type == "file":
        return str(file_path)
    elif item_type in ["class", "interface"]:
        return f"{file_path}::{item_name}"
    elif item_type in ["function", "method"]:
        if parent_name: # It's a method
            return f"{file_path}::{parent_name}::{item_name}"
        else: # It's a global function
            return f"{file_path}::{item_name}"
    return None

def parse_cpp_include(include_string, current_file_path):
    """
    Parses a C++ include string to find the target file path.
    This is a simplified example and might need to be more robust
    based on your project's include paths.
    """
    match = re.search(r'#include\s*["<]([^">]+)[">]', include_string)
    if match:
        included_file = match.group(1)
        # Resolve relative paths
        if not Path(included_file).is_absolute():
            # This is a very basic resolution. Real projects have include directories.
            # For now, assume it's relative to the current file's directory or a known include root.
            # A more sophisticated approach would check against a list of include paths.
            current_dir = Path(current_file_path).parent
            resolved_path = (current_dir / included_file).resolve()
            # You might want to check if resolved_path exists and is within your source_dir
            return str(resolved_path)
    return None


# --- Main Graph Building Logic ---

def build_codebase_graph(context_dir, source_root_dir):
    graph = nx.DiGraph()
    all_contexts = {}
    symbol_table = {} # Maps simple names to a list of fully qualified node IDs

    # Step 1: Load all context files and populate initial symbol table
    for root, _, files in os.walk(context_dir):
        for file_name in files:
            if file_name.endswith(".context"):
                file_path_in_context_data = os.path.join(root, file_name)
                with open(file_path_in_context_data, 'r', encoding='utf-8') as f:
                    try:
                        data = json.load(f)
                        # The 'file_path' in the JSON is the original source file path
                        original_source_path = data.get("file_path")
                        if not original_source_path:
                            print(f"Warning: No 'file_path' in {file_path_in_context_data}, skipping.")
                            continue
                        
                        # Normalize original_source_path to be absolute for consistency
                        # Assuming original_source_path might be relative to some unknown base
                        # or already absolute. If it's meant to be relative to source_root_dir:
                        if not Path(original_source_path).is_absolute():
                             original_source_path = str((Path(source_root_dir) / original_source_path).resolve())
                        else:
                             original_source_path = str(Path(original_source_path).resolve())


                        all_contexts[original_source_path] = data
                        
                        # Add file node
                        file_node_id = get_node_id(original_source_path, "file")
                        graph.add_node(file_node_id, type="file", label=Path(original_source_path).name, path=original_source_path)

                        # Populate symbol table with defined items
                        for class_info in data.get("classes", []):
                            class_name = class_info.get("metadata", {}).get("name")
                            if class_name:
                                node_id = get_node_id(original_source_path, "class", class_name)
                                symbol_table.setdefault(class_name, []).append(node_id)
                                for func_info in class_info.get("functions", []): # If LLM puts methods inside class block
                                    func_name = func_info.get("metadata", {}).get("name")
                                    if func_name:
                                        method_node_id = get_node_id(original_source_path, "method", func_name, class_name)
                                        symbol_table.setdefault(func_name, []).append(method_node_id)
                                        symbol_table.setdefault(f"{class_name}::{func_name}", []).append(method_node_id)


                        for func_info in data.get("functions", []):
                            func_name = func_info.get("metadata", {}).get("name")
                            parent_class = func_info.get("metadata", {}).get("parent") # Could be None
                            if func_name:
                                node_id = get_node_id(original_source_path, "method" if parent_class else "function", func_name, parent_class)
                                symbol_table.setdefault(func_name, []).append(node_id)
                                if parent_class:
                                    symbol_table.setdefault(f"{parent_class}::{func_name}", []).append(node_id)

                    except json.JSONDecodeError as e:
                        print(f"Error decoding JSON from {file_path_in_context_data}: {e}")
                    except Exception as e:
                        print(f"Error processing file {file_path_in_context_data}: {e}")


    # Step 2: Add nodes and edges based on contexts
    for file_path, data in all_contexts.items():
        current_file_node_id = get_node_id(file_path, "file")

        # Add class nodes and containment edges
        for class_info in data.get("classes", []):
            class_meta = class_info.get("metadata", {})
            class_name = class_meta.get("name")
            if class_name:
                class_node_id = get_node_id(file_path, "class", class_name)
                graph.add_node(class_node_id, type="class", label=class_name, file=file_path)
                graph.add_edge(current_file_node_id, class_node_id, type="contains")

                # Inheritance
                extends_class = class_meta.get("extends")
                if extends_class:
                    # Resolve extends_class name (this is simplified)
                    # A real system needs to search symbol_table and handle ambiguity
                    potential_parents = symbol_table.get(extends_class, [])
                    for parent_id in potential_parents: # Could be multiple if name isn't unique
                        if graph.has_node(parent_id): # Ensure target node exists
                             graph.add_edge(class_node_id, parent_id, type="inherits_from")
                        # else: print(f"Warning: Parent class {extends_class} for {class_node_id} not found in graph.")


                # Interface Implementation
                for impl_interface in class_meta.get("implements", []):
                    potential_interfaces = symbol_table.get(impl_interface, [])
                    for interface_id in potential_interfaces:
                        if graph.has_node(interface_id):
                            graph.add_edge(class_node_id, interface_id, type="implements")
                        # else: print(f"Warning: Interface {impl_interface} for {class_node_id} not found.")


                # Class-level dependencies
                for dep_name in class_meta.get("dependencies", []):
                    potential_deps = symbol_table.get(dep_name, [])
                    for dep_id in potential_deps:
                        if graph.has_node(dep_id):
                             graph.add_edge(class_node_id, dep_id, type="uses")


        # Add function/method nodes and containment/dependency edges
        for func_info in data.get("functions", []):
            func_meta = func_info.get("metadata", {})
            func_name = func_meta.get("name")
            parent_class_name = func_meta.get("parent")
            if func_name:
                func_node_id = get_node_id(file_path, "method" if parent_class_name else "function", func_name, parent_class_name)
                graph.add_node(func_node_id, type="method" if parent_class_name else "function", label=func_name, file=file_path)

                if parent_class_name:
                    parent_class_node_id = get_node_id(file_path, "class", parent_class_name)
                    if graph.has_node(parent_class_node_id): # Ensure parent class node exists
                        graph.add_edge(parent_class_node_id, func_node_id, type="contains_method")
                else:
                    graph.add_edge(current_file_node_id, func_node_id, type="contains_function")

                # Function/method dependencies
                for dep_name in func_meta.get("dependencies", []):
                    # Simple resolution: search globally. Could be refined.
                    potential_deps = symbol_table.get(dep_name, [])
                    for dep_id in potential_deps: # This might create many edges if name is common
                        if graph.has_node(dep_id) and dep_id != func_node_id : # Ensure target exists and not self-loop
                            graph.add_edge(func_node_id, dep_id, type="calls_or_uses")
                        # else: print(f"Warning: Dependency '{dep_name}' for '{func_node_id}' not found or is self.")


        # Process imports for file-level dependencies
        imports_block = data.get("imports", {})
        import_code = imports_block.get("code", "")
        if import_code: # Import code is expected to be a single string with CDATA
            actual_code_lines = import_code.splitlines()
            for line in actual_code_lines:
                line = line.strip()
                if line.startswith("#include"): # Basic C++ include parsing
                    target_file_str = parse_cpp_include(line, file_path)
                    if target_file_str:
                        # Check if this target_file_str corresponds to a known file node
                        # Need to normalize target_file_str similar to how keys in all_contexts were normalized
                        normalized_target_path = str(Path(target_file_str).resolve())

                        if normalized_target_path in all_contexts: # Check if we have context for it
                            target_file_node_id = get_node_id(normalized_target_path, "file")
                            if graph.has_node(target_file_node_id):
                                graph.add_edge(current_file_node_id, target_file_node_id, type="imports")
                            # else: print(f"Warning: Target import file node {target_file_node_id} not in graph though context exists.")
                        # else:
                        #     # It's an import of a file we don't have context for (e.g. system header)
                        #     # Optionally, add a node for it if you want to represent external dependencies
                        #     ext_node_id = f"external::{Path(target_file_str).name}"
                        #     if not graph.has_node(ext_node_id):
                        #         graph.add_node(ext_node_id, type="external_header", label=Path(target_file_str).name)
                        #     graph.add_edge(current_file_node_id, ext_node_id, type="imports_external")
                        #     pass # print(f"Info: Imported file {target_file_str} not found in parsed contexts.")

    return graph

def visualize_graph_matplotlib(graph, output_path="codebase_graph.png"):
    if not graph.nodes:
        print("Graph is empty, nothing to visualize.")
        return

    plt.figure(figsize=(20, 20)) # Adjust size as needed
    # Use a layout that works well for complex graphs, e.g., spring_layout or kamada_kawai_layout
    try:
        # pos = nx.spring_layout(graph, k=0.15, iterations=20)
        pos = nx.kamada_kawai_layout(graph) # Can be slow for very large graphs
    except Exception as e:
        print(f"Layout failed ({e}), falling back to random_layout.")
        pos = nx.random_layout(graph)

    # Node colors by type
    color_map = []
    for node in graph:
        node_type = graph.nodes[node].get('type', 'unknown')
        if node_type == 'file':
            color_map.append('skyblue')
        elif node_type == 'class':
            color_map.append('lightgreen')
        elif node_type == 'interface':
            color_map.append('palegreen')
        elif node_type == 'function':
            color_map.append('salmon')
        elif node_type == 'method':
            color_map.append('lightcoral')
        else:
            color_map.append('grey')

    labels = {n: graph.nodes[n].get('label', Path(n).name) for n in graph.nodes()}
    nx.draw(graph, pos, with_labels=True, labels=labels, node_size=500, node_color=color_map, font_size=8, arrowsize=10)
    
    # Edge labels (optional, can make graph very cluttered)
    # edge_labels = nx.get_edge_attributes(graph, 'type')
    # nx.draw_networkx_edge_labels(graph, pos, edge_labels=edge_labels, font_size=6)

    plt.title("Codebase Structure Graph")
    plt.savefig(output_path)
    plt.close()
    print(f"Graph saved to {output_path} using Matplotlib.")


def visualize_graph_graphviz(graph, output_path="codebase_graph_gv.png", prog="dot"):
    if not graph.nodes:
        print("Graph is empty, nothing to visualize with Graphviz.")
        return
    try:
        pydot_graph = to_pydot(graph)

        # Customize appearance for Graphviz (optional)
        for node_id, attrs in graph.nodes(data=True):
            pydot_node = pydot_graph.get_node(str(node_id)) # pydot nodes are strings
            if pydot_node and isinstance(pydot_node, list): pydot_node = pydot_node[0] # get_node can return a list

            if pydot_node:
                node_label = attrs.get('label', Path(node_id).name)
                pydot_node.set_label(node_label) # Ensure label is set
                node_type = attrs.get('type', 'unknown')
                if node_type == 'file':
                    pydot_node.set_shape('box')
                    pydot_node.set_fillcolor('skyblue')
                    pydot_node.set_style('filled')
                elif node_type == 'class':
                    pydot_node.set_shape('ellipse')
                    pydot_node.set_fillcolor('lightgreen')
                    pydot_node.set_style('filled')
                elif node_type == 'function' or node_type == 'method':
                    pydot_node.set_shape('ellipse')
                    pydot_node.set_fillcolor('salmon')
                    pydot_node.set_style('filled')
        
        for u, v, attrs in graph.edges(data=True):
             pydot_edge_list = pydot_graph.get_edge(str(u), str(v))
             if pydot_edge_list: # get_edge returns a list of edges
                 for pydot_edge in pydot_edge_list:
                     edge_type = attrs.get('type', '')
                     if edge_type == "imports": pydot_edge.set_color("blue")
                     elif edge_type == "inherits_from": pydot_edge.set_color("green"); pydot_edge.set_arrowhead("empty")
                     elif edge_type == "calls_or_uses": pydot_edge.set_color("red"); pydot_edge.set_style("dashed")
                     pydot_edge.set_label(edge_type) # Add edge type as label
                     pydot_edge.set_fontsize(8)


        pydot_graph.write_png(output_path, prog=prog) # prog can be dot, neato, fdp, sfdp, twopi, circo
        print(f"Graph saved to {output_path} using Graphviz (prog={prog}).")
    except ImportError:
        print("Graphviz (pydot) not installed. Skipping Graphviz visualization.")
    except Exception as e:
        print(f"Error during Graphviz visualization: {e}")


def main():
    parser = argparse.ArgumentParser(description="Build and visualize a codebase graph from .context files.")
    parser.add_argument("--context-dir", required=True, help="Directory containing the .context JSON files.")
    parser.add_argument("--source-root", required=True, help="Root directory of the original source code (for resolving relative paths).")
    parser.add_argument("--output-png", default="codebase_graph.png", help="Output path for the graph PNG (Matplotlib).")
    parser.add_argument("--output-gv-png", default="codebase_graph_gv.png", help="Output path for the graph PNG (Graphviz).")
    parser.add_argument("--output-gexf", default="codebase_graph.gexf", help="Output path for GEXF graph file (for Gephi).")
    parser.add_argument("--layout", default="kamada_kawai", help="Graphviz layout engine (e.g., dot, neato, fdp, sfdp, twopi, circo).")


    args = parser.parse_args()

    print(f"Building graph from contexts in: {args.context_dir}")
    print(f"Assuming source root for paths: {args.source_root}")
    graph = build_codebase_graph(args.context_dir, args.source_root)

    print(f"Graph built with {graph.number_of_nodes()} nodes and {graph.number_of_edges()} edges.")

    if graph.number_of_nodes() > 0:
        visualize_graph_matplotlib(graph, args.output_png)
        visualize_graph_graphviz(graph, args.output_gv_png, prog=args.layout)

        # Save graph data (e.g., for Gephi)
        try:
            nx.write_gexf(graph, args.output_gexf)
            print(f"Graph data saved to {args.output_gexf}")
        except Exception as e:
            print(f"Error saving GEXF: {e}")
    else:
        print("Graph is empty. No output generated.")

if __name__ == "__main__":
    main()