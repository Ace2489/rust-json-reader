# JSON Query CLI

A lightweight, efficient command-line tool for querying JSON files using a simple path syntax. This tool allows users to extract specific values from JSON documents without having to load the entire file into memory or use complex jq-style syntax.

## Features

- Fast JSON parsing with minimal dependencies
- Support for nested object traversal via dot notation
- Array access using bracket notation
- Robust error handling with informative messages
- Zero configuration required

## Usage

```bash
cargo run  -- /path/to/file.json path.to.value[0].property
```

## Example

Given a JSON file `data.json`:

```json
{
  "users": [
    {
      "name": "John",
      "details": {
        "age": 30,
        "roles": ["admin", "developer"]
      }
    }
  ]
}
```

You can query specific values:

```bash
# Get John's age
$ cargo run  -- /path/to/file.json  users[0].details.age
30

# Get John's first role
$ cargo run  --  /path/to/file.json users[0].details.roles[0]
"admin"
```

## Technical Implementation

This tool uses:
- Regular expressions for parsing query paths
- Pattern matching for type-safe JSON traversal
- Rust's powerful error handling for robust user feedback
- Minimal memory footprint for processing large JSON files

## Parsing and Execution

Borrowing some ideas from compiler construction, the execution process is as follows:
- Tokenize the query string into a list of query tokens.
- Execute the tokens, and traverse the JSON as instructed.
  
## License

MIT License - See [LICENSE](LICENSE) for details
