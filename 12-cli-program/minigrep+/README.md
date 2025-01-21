# Minigrep
___
Minigrep+ is a grep-inspired tool that searches through a given file line by line for a specified query (single word), and then returns the line(s) containing the query.
In the rust book chapter 12, a cli tool called minigrep is built, this tool expands on that by adding a help function and more advanced error handling.
### Usage
___
Run minigrep+ with:
```
cargo run -- [query] [file_path]
```
Put the wanted word in the query, and the specified file path in file_path.

For more help:
```
cargo run -- help
```