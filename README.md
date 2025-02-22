# jom

`jom` is a lightweight Rust library that converts JSON data into Markdown by replacing placeholders in a Markdown template with corresponding JSON values. It supports nested keys using dot notation (e.g., `{user.name}`) and now includes special syntax to easily render arrays and objects as Markdown list items.

## Overview

This library provides a single public function, `json_to_markdown`, which:

- Parses a JSON string.
- Searches for placeholders in a Markdown template.
- Replaces each placeholder with the corresponding value from the JSON data if available.
- Supports nested keys using dot notation (e.g., `{user.name}`).
- **New Feature:** Supports array/object destructuring via the `...{key}` syntax, which renders array elements as list items (using `- `) and object key/value pairs as list items in `key: value` format.
- Leaves the placeholder unchanged if the corresponding key does not exist.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
jom = "0.1.3"
```

## Usage

Below is a quick example demonstrating how to use the library with both standard placeholders and the new array/object destructuring syntax:

```rust
use jom::json_to_markdown;

fn main() -> serde_json::Result<()> {
    let json_data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "contact": ["+44 1234567", "+44 2345678"],
        "skills": {
            "languages": ["Rust", "Python", "JavaScript"],
            "tools": ["Git", "Docker", "Kubernetes"]
        }
    }
    "#;

    let markdown_template = r#"
    # {name}
    ## {age}
    ### Contact Details
    ...{contact}
    ### Skills
    #### Languages
    ...{skills.languages}
    #### Tools
    ...{skills.tools}
    "#;

    let rendered = json_to_markdown(json_data, markdown_template)?;
    println!("{}", rendered);
    Ok(())
}
```

In the example above:
- Standard placeholders like `{name}` and `{age}` are replaced directly.
- The `...{contact}` placeholder detects that `contact` is an array and converts each element into a Markdown list item.
- Similarly, `...{skills.languages}` and `...{skills.tools}` render their respective arrays as lists.

## API Reference

### `json_to_markdown`

```rust
pub fn json_to_markdown(json_data: &str, markdown: &str) -> serde_json::Result<String>
```

- **Parameters:**
  - `json_data`: A string slice containing valid JSON.
  - `markdown`: A string slice containing a Markdown template with placeholders in the form `{key}` or `{nested.key}`. For array or object destructuring, use the syntax `...{key}`.
  
- **Returns:**
  - A `Result<String, serde_json::Error>` where the `String` is the rendered Markdown if successful, or a `serde_json::Error` if JSON parsing fails.
  
- **Behavior:**
  - Replaces every placeholder found in the Markdown template with the corresponding value from the parsed JSON.
  - When a placeholder is prefixed with `...`, the function checks if the JSON value is an array or an object and renders each element as a Markdown list item.
  - Leaves the original placeholder unchanged if the key is not found in the JSON.

## Acknowledgements

This project was inspired by [vvhg1](https://github.com/vvhg1)'s [fractalparser](https://github.com/vvhg1/fractalparser) 🐍.

## License

This project is licensed under the [MIT License](LICENSE).
