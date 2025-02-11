# jom

`jom` is a lightweight Rust library that converts JSON data into Markdown by replacing placeholders in a Markdown template with corresponding JSON values. It supports nested keys using dot notation (e.g., `{user.name}`).

## Overview

This library provides a single public function, `json_to_markdown`, which:
- Parses a JSON string.
- Searches for placeholders in a Markdown template.
- Replaces each placeholder with the value from the JSON data if available.
- Leaves the placeholder unchanged if the corresponding key does not exist.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
jom = "0.1.3"
```

## Usage

Below is a quick example demonstrating how to use the library:

```rust
use jom::json_to_markdown;

fn main() -> serde_json::Result<()> {
    let json_data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": {
            "0": "+44 1234567",
            "1": "+44 2345678"
        },
        "address": {
            "street": "10 Downing Street",
            "city": "London",
            "postal_code": "SW1A 1AA"
        }
    }
    "#;

    let markdown_template = r#"
    # {name}
    ## {age}
    ### Contact Details
    - {phones.0}
    - {phones.1}
    ### Address
    - Street: {address.street}
    - City: {address.city}
    - Postal Code: {address.postal_code}
    "#;

    let rendered = json_to_markdown(json_data, markdown_template)?;
    println!("{}", rendered);
    Ok(())
}
```

## API Reference

### `json_to_markdown`

```rust
pub fn json_to_markdown(json_data: &str, markdown: &str) -> serde_json::Result<String>
```

- **Parameters:**
  - `json_data`: A string slice containing valid JSON.
  - `markdown`: A string slice containing a Markdown template with placeholders in the form `{key}` or `{nested.key}`.

- **Returns:**
  - A `Result<String, serde_json::Error>` where the `String` is the rendered Markdown if successful, or a `serde_json::Error` if JSON parsing fails.

- **Behavior:**
  - The function replaces every placeholder found in the `markdown` template with the corresponding value from the parsed JSON. If a key is not found in the JSON data, the original placeholder is retained.

## Acknowledgements

This project was inspired by [vvhg1](https://github.com/vvhg1)'s [fractal-lasagna](https://github.com/vvhg1/fractal-lasagna) 🥫.

## License

This project is licensed under the [MIT License](LICENSE).
