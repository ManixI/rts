#!/usr/bin/env python3
import os
import sys

CLEANUP_FILES = [
    ".ppm",
    ".png"
]
DIR_PATH = "../"

def delete_ppm_files():
    dir_path = os.path.abspath(target_path)

    if not os.path.isdir(dir_path):
        print(f"Error: '{target_path}' is not a valid directory.")
        return
    
    for filename in os.listdir(dir_path):
        for suffix in CLEANUP_FILES:
            if filename.endswith(suffix):
                file_path = os.path.join(script_dir, filename)
                try:
                    os.remove(file_path)
                    print(f"Deleted: {file_path}")
                except Exception as e:
                    print(f"Failed to delete {file_path}: {e}")

if __name__ == "__main__":
    delete_ppm_files(DIR_PATH)