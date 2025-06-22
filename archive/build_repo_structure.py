import os
import json

def build_tree(path):
    """
    Recursively builds a dictionary representing the folder structure.
    
    Each node in the tree has:
      - 'name': The name of the file or folder.
      - 'type': "folder" for directories or "file" for files.
      - 'children': A list of children nodes (only for folders).
    """
    tree = {"name": os.path.basename(path)}
    
    if os.path.isdir(path):
        tree["type"] = "folder"
        tree["children"] = []
        try:
            # Sort entries for a consistent output order
            for entry in sorted(os.listdir(path)):
                full_path = os.path.join(path, entry)
                tree["children"].append(build_tree(full_path))
        except PermissionError:
            # Handle folders that cannot be accessed due to permissions
            tree["children"].append({"name": "Permission Denied", "type": "error"})
    else:
        tree["type"] = "file"
        
    return tree

if __name__ == "__main__":
    folder_path = input("Enter the folder path: ")
    tree_structure = build_tree(folder_path)
    # Output the structure as pretty-printed JSON
    print(json.dumps(tree_structure, indent=4))
