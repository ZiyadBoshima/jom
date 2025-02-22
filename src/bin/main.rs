use jom::json_to_markdown;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // JSON data example
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "contact": ["+4400000000", "john@doe.com"],
            "skills": {
                "languages": ["Rust", "Python", "JavaScript"],
                "tools": ["Git", "Docker", "Kubernetes"]
            }
        }"#;

    // markdown example
    let markdown = r#"
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

    let rendered = json_to_markdown(data, markdown).unwrap_or("".to_string());

    println!("{}", rendered);

    Ok(())
}
