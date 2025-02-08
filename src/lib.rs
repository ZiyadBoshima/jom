use regex::Regex;
use serde_json::{self, Value};

/// Helper function to get nested JSON value for a given dot-separated key.
///
/// For example, given the key `"phones.0"`, it will attempt to access
/// `json["phones"]["0"]`.
fn get_nested_value<'a>(json: &'a Value, key: &str) -> Option<&'a Value> {
    let mut value = json;
    for k in key.split('.') {
        value = value.get(k)?;
    }
    Some(value)
}

/// Converts JSON data to markdown by replacing placeholders in the markdown template
/// with values extracted from the JSON data. Placeholders should be in the form `{key}`
/// or `{nested.key}`.
///
/// # Arguments
///
/// * `json_data` - A string slice containing the JSON data.
/// * `markdown` - A string slice containing the markdown template.
///
/// # Returns
///
/// A `Result` containing the rendered markdown as a `String` or a `serde_json::Error`.
///
/// # Example
///
/// ```rust
/// use jom::json_to_markdown;
///
/// let json_data = r#"
/// {
///     "name": "John Doe",
///     "age": 43,
///     "phones": {
///         "0": "+44 1234567",
///         "1": "+44 2345678"
///     }
/// }"#;
///
/// let markdown = r#"
/// # {name}
/// ## {age}
/// ### Contact Details
/// - {phones.0}
/// - {phones.1}
/// "#;
///
/// let rendered = json_to_markdown(json_data, markdown).unwrap();
/// println!("{}", rendered);
/// ```
///
pub fn json_to_markdown(json_data: &str, markdown: &str) -> serde_json::Result<String> {
    // Parse the JSON string into a serde_json::Value.
    let json: Value = serde_json::from_str(json_data)?;

    // Compile the regex to capture placeholders like {key} or {nested.key}.
    let re = Regex::new(r"\{([a-zA-Z0-9_.]+)\}").unwrap();

    // Replace each placeholder with the corresponding value from the JSON.
    let rendered = re.replace_all(markdown, |caps: &regex::Captures| {
        // Extract the key name from the captured group.
        let key = caps.get(1).unwrap().as_str();
        // Look up the key in the JSON. If not found, leave the original placeholder.
        match get_nested_value(&json, key) {
            Some(value) => value.to_string().replace("\"", ""),
            None => caps.get(0).unwrap().as_str().to_string(),
        }
    });

    Ok(rendered.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_markdown() {
        let json_data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": {
                "0": "+44 1234567",
                "1": "+44 2345678"
            }
        }"#;

        let markdown = r#"
        # {name}
        ## {age}
        ### Contact Details
        - {phones.0}
        - {phones.1}
        "#;

        let rendered = json_to_markdown(json_data, markdown).unwrap();

        // Ensure the output contains the expected values.
        assert!(rendered.contains("John Doe"));
        assert!(rendered.contains("43"));
        assert!(rendered.contains("+44 1234567"));
        assert!(rendered.contains("+44 2345678"));
    }
}
