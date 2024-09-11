#!/bin/bash

# Output markdown file
output_file="output.md"

# Define directories and files to ignore
ignore_dirs=("target" "ripi" ".git" "tmp")
ignore_files=("README.md" ".gitignore" "Cargo.lock" ".dockerignore" "generate_markdown.sh")

# Function to check if a file or directory should be ignored
should_ignore() {
    local file="$1"

    # Check for ignored directories
    for dir in "${ignore_dirs[@]}"; do
        if [[ "$file" == */"$dir"/* ]]; then
            return 0  # ignore this file
        fi
    done

    # Check for ignored files
    for ignore_file in "${ignore_files[@]}"; do
        if [[ "$file" == *"$ignore_file" ]]; then
            return 0  # ignore this file
        fi
    done

    return 1  # don't ignore
}

# Clear the output file if it exists
> "$output_file"

# Recursively find all files, ignoring specified directories and files
find . -type f | while read -r file; do
    # Check if the file should be ignored
    if should_ignore "$file"; then
        continue
    fi

    # Add the file path as a header in markdown format
    echo "# ${file#./}" >> "$output_file"
    echo "" >> "$output_file"

    # Append the content of the file
    cat "$file" >> "$output_file"

    # Add markdown separator
    echo "" >> "$output_file"
    echo "---" >> "$output_file"
    echo "" >> "$output_file"
done

echo "Markdown file created: $output_file"
