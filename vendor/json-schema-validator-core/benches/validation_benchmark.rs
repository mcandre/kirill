use criterion::{black_box, criterion_group, criterion_main, Criterion};
use json_schema_validator_core::{JsonSchemaValidator, ValidationOptions};
use serde_json::{json, Value};

fn simple_object_validation(c: &mut Criterion) {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "age": {"type": "integer", "minimum": 0}
        },
        "required": ["name"]
    });

    let instance = json!({
        "name": "John Doe",
        "age": 30
    });

    let validator = JsonSchemaValidator::new(schema, ValidationOptions::default()).unwrap();

    c.bench_function("simple object validation", |b| {
        b.iter(|| validator.is_valid(black_box(&instance)))
    });
}

fn complex_nested_validation(c: &mut Criterion) {
    let schema = json!({
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "profile": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string", "minLength": 1, "maxLength": 50},
                            "email": {"type": "string", "format": "email"},
                            "age": {"type": "integer", "minimum": 13, "maximum": 120}
                        },
                        "required": ["name", "email"]
                    },
                    "preferences": {
                        "type": "object",
                        "properties": {
                            "theme": {"enum": ["light", "dark", "auto"]},
                            "notifications": {"type": "boolean"},
                            "tags": {
                                "type": "array",
                                "items": {"type": "string"},
                                "maxItems": 10,
                                "uniqueItems": true
                            }
                        }
                    }
                },
                "required": ["profile"]
            }
        },
        "required": ["user"]
    });

    let instance = json!({
        "user": {
            "profile": {
                "name": "Alice Johnson",
                "email": "alice@example.com",
                "age": 28
            },
            "preferences": {
                "theme": "dark",
                "notifications": true,
                "tags": ["developer", "rust", "json-schema"]
            }
        }
    });

    let validator = JsonSchemaValidator::new(schema, ValidationOptions::default()).unwrap();

    c.bench_function("complex nested validation", |b| {
        b.iter(|| validator.is_valid(black_box(&instance)))
    });
}

fn large_array_validation(c: &mut Criterion) {
    let schema = json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "id": {"type": "integer"},
                "value": {"type": "string", "minLength": 1}
            },
            "required": ["id", "value"]
        },
        "minItems": 1,
        "maxItems": 1000
    });

    let instance: Value = json!((0..100).map(|i| json!({
        "id": i,
        "value": format!("item_{}", i)
    })).collect::<Vec<_>>());

    let validator = JsonSchemaValidator::new(schema, ValidationOptions::default()).unwrap();

    c.bench_function("large array validation", |b| {
        b.iter(|| validator.is_valid(black_box(&instance)))
    });
}

fn string_format_validation(c: &mut Criterion) {
    let schema = json!({
        "type": "object",
        "properties": {
            "email": {"type": "string", "format": "email"},
            "uri": {"type": "string", "format": "uri"},
            "date": {"type": "string", "format": "date"},
            "uuid": {"type": "string", "format": "uuid"}
        },
        "required": ["email", "uri", "date", "uuid"]
    });

    let instance = json!({
        "email": "user@example.com",
        "uri": "https://example.com/path",
        "date": "2023-12-25",
        "uuid": "123e4567-e89b-12d3-a456-426614174000"
    });

    let validator = JsonSchemaValidator::new(schema, ValidationOptions::default()).unwrap();

    c.bench_function("string format validation", |b| {
        b.iter(|| validator.is_valid(black_box(&instance)))
    });
}

fn validation_with_errors(c: &mut Criterion) {
    let schema = json!({
        "type": "object",
        "properties": {
            "name": {"type": "string", "minLength": 3},
            "age": {"type": "integer", "minimum": 0, "maximum": 150},
            "email": {"type": "string", "format": "email"}
        },
        "required": ["name", "age", "email"]
    });

    let invalid_instance = json!({
        "name": "Jo",  // too short
        "age": -5,     // negative
        "email": "invalid-email"  // invalid format
    });

    let validator = JsonSchemaValidator::new(schema, ValidationOptions::default()).unwrap();

    c.bench_function("validation with errors", |b| {
        b.iter(|| {
            let errors = validator.validate(black_box(&invalid_instance));
            black_box(errors.len())
        })
    });
}

fn schema_compilation(c: &mut Criterion) {
    let schema = json!({
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "name": {"type": "string", "pattern": "^[A-Za-z ]+$"},
                    "email": {"type": "string", "format": "email"},
                    "age": {"type": "integer", "minimum": 0}
                },
                "required": ["name", "email"]
            },
            "settings": {
                "type": "object",
                "properties": {
                    "theme": {"enum": ["light", "dark"]},
                    "language": {"type": "string", "minLength": 2, "maxLength": 5}
                }
            }
        },
        "required": ["user"]
    });

    c.bench_function("schema compilation", |b| {
        b.iter(|| {
            let validator = JsonSchemaValidator::new(
                black_box(schema.clone()), 
                ValidationOptions::default()
            ).unwrap();
            black_box(validator)
        })
    });
}

criterion_group!(
    benches,
    simple_object_validation,
    complex_nested_validation,
    large_array_validation,
    string_format_validation,
    validation_with_errors,
    schema_compilation
);
criterion_main!(benches);
