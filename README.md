# minigrep
A minimalist grep written in Rust

## Installation

### For Unix/Linux
Run the following command in your terminal:

### For Windows
Download the minigrep.exe from the Releases page and copy it to a directory in your PATH.

```bash
curl -sSL https://raw.githubusercontent.com/yourusername/minigrep/main/install.sh | sh
```

## Basic Usage
bash
```
minigrep <query> <filepath>
```
<query>: The text to search for.
<filepath>: The path to the file to search in.

Options
1. Ignore Case
To ignore case when searching, use the ignore_case=true:
bash
```
ignore_case=true minigrep <query> <filepath> 
```

When this option is set to true, the search will be case-insensitive.

2. Count Matching Lines
To count the number of lines that contain the query text, use the ROWS=1:
bash
```
ROWS=1 minigrep <query> <filepath>
```

When this option is set to 1, minigrep will output the count of lines that match the query.

3. Count Total Lines
To count the total number of lines in the file, use the ROWSOFFILE=1:
bash
```
ROWSOFFILE=1 minigrep <query> <filepath>
```

When this option is set to 1, minigrep will output the total count of lines in the file.

## Supported File Types
minigrep currently supports searching in the following file types:
Markdown files (.md)
Plain text files (.txt)
The tool will parse the text content of these files, ignoring any Markdown syntax or formatting.