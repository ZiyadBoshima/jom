use jom::json_to_markdown;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": {
                "0": "+44 1234567",
                "1": "+44 2345678"
            }
        }"#;

    // markdown example
    let markdown = r#"
        # {name}
        ## {age}
        ### Contact Details
        - {phones.0}
        - {phones.1}
    "#;

    let rendered = json_to_markdown(data, markdown).unwrap_or("".to_string());

    println!("{}", rendered);

    Ok(())
}
