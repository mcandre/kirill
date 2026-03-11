# JSON Schema Validator Core Examples

## Simple Validation Example

```rust
use json_schema_validator_core::{JsonSchemaValidator, ValidationOptions};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "number", "minimum": 0}
        },
        "required": ["name"]
    });

    let valid_data = json!({
        "name": "Alice",
        "age": 30
    });

    let invalid_data = json!({
        "age": -5
    });

    let validator = JsonSchemaValidator::new(schema, ValidationOptions::default())?;

    // Validate and print results
    println!("Valid data: {}", validator.is_valid(&valid_data));
    
    let errors = validator.validate(&invalid_data);
    println!("Invalid data errors:");
    for error in errors {
        println!("  - {}: {}", error.instance_path, error.message);
    }

    Ok(())
}
```

## Advanced Configuration Example

```rust
use json_schema_validator_core::{JsonSchemaValidator, ValidationOptions, SchemaDraft};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create custom format validator
    let mut custom_formats = HashMap::new();
    custom_formats.insert("phone".to_string(), |s: &str| -> bool {
        s.len() >= 10 && s.chars().all(|c| c.is_ascii_digit() || c == '-' || c == '(' || c == ')' || c == ' ')
    });

    let options = ValidationOptions {
        draft: SchemaDraft::Draft7,
        custom_formats,
        short_circuit: false, // Collect all errors
        collect_annotations: true,
    };

    let schema = json!({
        "type": "object",
        "properties": {
            "phone": {"type": "string", "format": "phone"},
            "email": {"type": "string", "format": "email"}
        }
    });

    let data = json!({
        "phone": "555-123-4567",
        "email": "user@example.com"
    });

    let validator = JsonSchemaValidator::new(schema, options)?;
    println!("Is valid: {}", validator.is_valid(&data));

    Ok(())
}
```
