use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wasm_bindgen::prelude::*;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub instance_path: String,
    pub schema_path: String,
    pub keyword: String,
    pub message: String,
    pub instance_value: Option<Value>,
    pub schema_value: Option<Value>,
}

pub struct ValidationOptions {
    pub draft: SchemaDraft,
    pub custom_formats: HashMap<String, fn(&str) -> bool>,
    pub short_circuit: bool,
    pub collect_annotations: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum SchemaDraft {
    Draft4,
    Draft6,
    Draft7,
    Draft201909,
    Draft202012,
}

impl Default for ValidationOptions {
    fn default() -> Self {
        Self {
            draft: SchemaDraft::Draft7,
            custom_formats: HashMap::new(),
            short_circuit: false,
            collect_annotations: false,
        }
    }
}

pub struct JsonSchemaValidator {
    schema: Value,
    options: ValidationOptions,
}

impl JsonSchemaValidator {
    pub fn new(schema: Value, options: ValidationOptions) -> Result<Self, ValidationError> {
        let validator = Self {
            schema,
            options,
        };
        
        // Pre-validate the schema
        validator.validate_schema()?;
        Ok(validator)
    }

    pub fn validate(&self, instance: &Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        self.validate_recursive(instance, &self.schema, "", "", &mut errors);
        errors
    }

    pub fn is_valid(&self, instance: &Value) -> bool {
        let mut errors = Vec::new();
        self.validate_recursive(instance, &self.schema, "", "", &mut errors);
        errors.is_empty()
    }

    fn validate_schema(&self) -> Result<(), ValidationError> {
        // Basic schema validation - ensure it's a valid JSON schema
        if !self.schema.is_object() {
            return Err(ValidationError {
                instance_path: "".to_string(),
                schema_path: "".to_string(),
                keyword: "schema".to_string(),
                message: "Schema must be an object".to_string(),
                instance_value: None,
                schema_value: Some(self.schema.clone()),
            });
        }
        Ok(())
    }

    fn validate_recursive(
        &self,
        instance: &Value,
        schema: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if self.options.short_circuit && !errors.is_empty() {
            return;
        }

        let schema_obj = match schema.as_object() {
            Some(obj) => obj,
            None => return,
        };

        // Type validation
        if let Some(type_value) = schema_obj.get("type") {
            self.validate_type(instance, type_value, instance_path, schema_path, errors);
        }

        // String validations
        if instance.is_string() {
            let string_val = instance.as_str().unwrap();
            
            if let Some(min_length) = schema_obj.get("minLength") {
                self.validate_min_length(string_val, min_length, instance_path, schema_path, errors);
            }
            
            if let Some(max_length) = schema_obj.get("maxLength") {
                self.validate_max_length(string_val, max_length, instance_path, schema_path, errors);
            }
            
            if let Some(pattern) = schema_obj.get("pattern") {
                self.validate_pattern(string_val, pattern, instance_path, schema_path, errors);
            }
            
            if let Some(format) = schema_obj.get("format") {
                self.validate_format(string_val, format, instance_path, schema_path, errors);
            }
        }

        // Number validations
        if instance.is_number() {
            let num_val = instance.as_f64().unwrap();
            
            if let Some(minimum) = schema_obj.get("minimum") {
                self.validate_minimum(num_val, minimum, instance_path, schema_path, errors);
            }
            
            if let Some(maximum) = schema_obj.get("maximum") {
                self.validate_maximum(num_val, maximum, instance_path, schema_path, errors);
            }
            
            if let Some(multiple_of) = schema_obj.get("multipleOf") {
                self.validate_multiple_of(num_val, multiple_of, instance_path, schema_path, errors);
            }
        }

        // Array validations
        if let Some(array) = instance.as_array() {
            if let Some(min_items) = schema_obj.get("minItems") {
                self.validate_min_items(array, min_items, instance_path, schema_path, errors);
            }
            
            if let Some(max_items) = schema_obj.get("maxItems") {
                self.validate_max_items(array, max_items, instance_path, schema_path, errors);
            }
            
            if let Some(unique_items) = schema_obj.get("uniqueItems") {
                if unique_items.as_bool().unwrap_or(false) {
                    self.validate_unique_items(array, instance_path, schema_path, errors);
                }
            }
            
            if let Some(items_schema) = schema_obj.get("items") {
                self.validate_array_items(array, items_schema, instance_path, schema_path, errors);
            }
        }

        // Object validations
        if let Some(object) = instance.as_object() {
            if let Some(min_properties) = schema_obj.get("minProperties") {
                self.validate_min_properties(object, min_properties, instance_path, schema_path, errors);
            }
            
            if let Some(max_properties) = schema_obj.get("maxProperties") {
                self.validate_max_properties(object, max_properties, instance_path, schema_path, errors);
            }
            
            if let Some(required) = schema_obj.get("required") {
                self.validate_required(object, required, instance_path, schema_path, errors);
            }
            
            if let Some(properties) = schema_obj.get("properties") {
                self.validate_object_properties(object, properties, instance_path, schema_path, errors);
            }
            
            if let Some(additional_properties) = schema_obj.get("additionalProperties") {
                let properties = schema_obj.get("properties");
                self.validate_additional_properties(object, properties, additional_properties, instance_path, schema_path, errors);
            }
        }

        // Enum validation
        if let Some(enum_values) = schema_obj.get("enum") {
            self.validate_enum(instance, enum_values, instance_path, schema_path, errors);
        }

        // Const validation
        if let Some(const_value) = schema_obj.get("const") {
            self.validate_const(instance, const_value, instance_path, schema_path, errors);
        }
    }

    fn validate_type(
        &self,
        instance: &Value,
        type_value: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        let expected_types = match type_value {
            Value::String(type_str) => vec![type_str.as_str()],
            Value::Array(type_array) => {
                type_array.iter().filter_map(|v| v.as_str()).collect()
            }
            _ => return,
        };

        let instance_type = get_json_type(instance);
        let mut type_matches = false;
        
        for expected_type in &expected_types {
            match expected_type {
                &"integer" => {
                    if let Value::Number(n) = instance {
                        if n.is_i64() || n.is_u64() {
                            type_matches = true;
                            break;
                        }
                    }
                }
                &"number" => {
                    if instance.is_number() {
                        type_matches = true;
                        break;
                    }
                }
                _ => {
                    if instance_type == *expected_type {
                        type_matches = true;
                        break;
                    }
                }
            }
        }
        
        if !type_matches {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/type", schema_path),
                keyword: "type".to_string(),
                message: format!("Expected type {}, got {}", 
                    if expected_types.len() == 1 { 
                        expected_types[0].to_string() 
                    } else { 
                        format!("one of [{}]", expected_types.join(", ")) 
                    },
                    instance_type
                ),
                instance_value: Some(instance.clone()),
                schema_value: Some(type_value.clone()),
            });
        }
    }

    fn validate_min_length(
        &self,
        string_val: &str,
        min_length: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(min_len) = min_length.as_u64() {
            if string_val.chars().count() < min_len as usize {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/minLength", schema_path),
                    keyword: "minLength".to_string(),
                    message: format!("String length {} is less than minimum {}", 
                        string_val.chars().count(), min_len),
                    instance_value: Some(Value::String(string_val.to_string())),
                    schema_value: Some(min_length.clone()),
                });
            }
        }
    }

    fn validate_max_length(
        &self,
        string_val: &str,
        max_length: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(max_len) = max_length.as_u64() {
            if string_val.chars().count() > max_len as usize {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/maxLength", schema_path),
                    keyword: "maxLength".to_string(),
                    message: format!("String length {} exceeds maximum {}", 
                        string_val.chars().count(), max_len),
                    instance_value: Some(Value::String(string_val.to_string())),
                    schema_value: Some(max_length.clone()),
                });
            }
        }
    }

    fn validate_pattern(
        &self,
        string_val: &str,
        pattern: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(pattern_str) = pattern.as_str() {
            match Regex::new(pattern_str) {
                Ok(regex) => {
                    if !regex.is_match(string_val) {
                        errors.push(ValidationError {
                            instance_path: instance_path.to_string(),
                            schema_path: format!("{}/pattern", schema_path),
                            keyword: "pattern".to_string(),
                            message: format!("String '{}' does not match pattern '{}'", 
                                string_val, pattern_str),
                            instance_value: Some(Value::String(string_val.to_string())),
                            schema_value: Some(pattern.clone()),
                        });
                    }
                }
                Err(_) => {
                    errors.push(ValidationError {
                        instance_path: instance_path.to_string(),
                        schema_path: format!("{}/pattern", schema_path),
                        keyword: "pattern".to_string(),
                        message: format!("Invalid regex pattern: '{}'", pattern_str),
                        instance_value: Some(Value::String(string_val.to_string())),
                        schema_value: Some(pattern.clone()),
                    });
                }
            }
        }
    }

    fn validate_format(
        &self,
        string_val: &str,
        format: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(format_str) = format.as_str() {
            let is_valid = match format_str {
                "email" => validate_email(string_val),
                "uri" => validate_uri(string_val),
                "date" => validate_date(string_val),
                "date-time" => validate_datetime(string_val),
                "ipv4" => validate_ipv4(string_val),
                "ipv6" => validate_ipv6(string_val),
                "uuid" => validate_uuid(string_val),
                custom_format => {
                    if let Some(validator) = self.options.custom_formats.get(custom_format) {
                        validator(string_val)
                    } else {
                        true // Unknown formats are ignored
                    }
                }
            };

            if !is_valid {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/format", schema_path),
                    keyword: "format".to_string(),
                    message: format!("String '{}' is not a valid {}", string_val, format_str),
                    instance_value: Some(Value::String(string_val.to_string())),
                    schema_value: Some(format.clone()),
                });
            }
        }
    }

    fn validate_minimum(
        &self,
        num_val: f64,
        minimum: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(min_val) = minimum.as_f64() {
            if num_val < min_val {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/minimum", schema_path),
                    keyword: "minimum".to_string(),
                    message: format!("Value {} is less than minimum {}", num_val, min_val),
                    instance_value: Some(Value::Number(serde_json::Number::from_f64(num_val).unwrap())),
                    schema_value: Some(minimum.clone()),
                });
            }
        }
    }

    fn validate_maximum(
        &self,
        num_val: f64,
        maximum: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(max_val) = maximum.as_f64() {
            if num_val > max_val {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/maximum", schema_path),
                    keyword: "maximum".to_string(),
                    message: format!("Value {} exceeds maximum {}", num_val, max_val),
                    instance_value: Some(Value::Number(serde_json::Number::from_f64(num_val).unwrap())),
                    schema_value: Some(maximum.clone()),
                });
            }
        }
    }

    fn validate_multiple_of(
        &self,
        num_val: f64,
        multiple_of: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(divisor) = multiple_of.as_f64() {
            if divisor > 0.0 && (num_val / divisor).fract() != 0.0 {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/multipleOf", schema_path),
                    keyword: "multipleOf".to_string(),
                    message: format!("Value {} is not a multiple of {}", num_val, divisor),
                    instance_value: Some(Value::Number(serde_json::Number::from_f64(num_val).unwrap())),
                    schema_value: Some(multiple_of.clone()),
                });
            }
        }
    }

    fn validate_min_items(
        &self,
        array: &[Value],
        min_items: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(min_len) = min_items.as_u64() {
            if array.len() < min_len as usize {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/minItems", schema_path),
                    keyword: "minItems".to_string(),
                    message: format!("Array length {} is less than minimum {}", array.len(), min_len),
                    instance_value: Some(Value::Array(array.to_vec())),
                    schema_value: Some(min_items.clone()),
                });
            }
        }
    }

    fn validate_max_items(
        &self,
        array: &[Value],
        max_items: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(max_len) = max_items.as_u64() {
            if array.len() > max_len as usize {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/maxItems", schema_path),
                    keyword: "maxItems".to_string(),
                    message: format!("Array length {} exceeds maximum {}", array.len(), max_len),
                    instance_value: Some(Value::Array(array.to_vec())),
                    schema_value: Some(max_items.clone()),
                });
            }
        }
    }

    fn validate_unique_items(
        &self,
        array: &[Value],
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        let mut seen = std::collections::HashSet::new();
        for (i, item) in array.iter().enumerate() {
            let item_str = serde_json::to_string(item).unwrap();
            if !seen.insert(item_str) {
                errors.push(ValidationError {
                    instance_path: format!("{}/{}", instance_path, i),
                    schema_path: format!("{}/uniqueItems", schema_path),
                    keyword: "uniqueItems".to_string(),
                    message: "Array contains duplicate items".to_string(),
                    instance_value: Some(item.clone()),
                    schema_value: Some(Value::Bool(true)),
                });
                break;
            }
        }
    }

    fn validate_array_items(
        &self,
        array: &[Value],
        items_schema: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        for (i, item) in array.iter().enumerate() {
            let item_path = format!("{}/{}", instance_path, i);
            let item_schema_path = format!("{}/items", schema_path);
            self.validate_recursive(item, items_schema, &item_path, &item_schema_path, errors);
        }
    }

    fn validate_min_properties(
        &self,
        object: &serde_json::Map<String, Value>,
        min_properties: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(min_props) = min_properties.as_u64() {
            if object.len() < min_props as usize {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/minProperties", schema_path),
                    keyword: "minProperties".to_string(),
                    message: format!("Object has {} properties, minimum is {}", object.len(), min_props),
                    instance_value: Some(Value::Object(object.clone())),
                    schema_value: Some(min_properties.clone()),
                });
            }
        }
    }

    fn validate_max_properties(
        &self,
        object: &serde_json::Map<String, Value>,
        max_properties: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(max_props) = max_properties.as_u64() {
            if object.len() > max_props as usize {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/maxProperties", schema_path),
                    keyword: "maxProperties".to_string(),
                    message: format!("Object has {} properties, maximum is {}", object.len(), max_props),
                    instance_value: Some(Value::Object(object.clone())),
                    schema_value: Some(max_properties.clone()),
                });
            }
        }
    }

    fn validate_required(
        &self,
        object: &serde_json::Map<String, Value>,
        required: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(required_array) = required.as_array() {
            for req_prop in required_array {
                if let Some(prop_name) = req_prop.as_str() {
                    if !object.contains_key(prop_name) {
                        errors.push(ValidationError {
                            instance_path: instance_path.to_string(),
                            schema_path: format!("{}/required", schema_path),
                            keyword: "required".to_string(),
                            message: format!("Missing required property '{}'", prop_name),
                            instance_value: Some(Value::Object(object.clone())),
                            schema_value: Some(required.clone()),
                        });
                    }
                }
            }
        }
    }

    fn validate_object_properties(
        &self,
        object: &serde_json::Map<String, Value>,
        properties: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(props_obj) = properties.as_object() {
            for (prop_name, prop_value) in object {
                if let Some(prop_schema) = props_obj.get(prop_name) {
                    let prop_path = format!("{}/{}", instance_path, prop_name);
                    let prop_schema_path = format!("{}/properties/{}", schema_path, prop_name);
                    self.validate_recursive(prop_value, prop_schema, &prop_path, &prop_schema_path, errors);
                }
            }
        }
    }

    fn validate_additional_properties(
        &self,
        object: &serde_json::Map<String, Value>,
        properties: Option<&Value>,
        additional_properties: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        let defined_props: std::collections::HashSet<&String> = properties
            .and_then(|p| p.as_object())
            .map(|obj| obj.keys().collect())
            .unwrap_or_default();

        for (prop_name, prop_value) in object {
            if !defined_props.contains(prop_name) {
                match additional_properties {
                    Value::Bool(false) => {
                        errors.push(ValidationError {
                            instance_path: format!("{}/{}", instance_path, prop_name),
                            schema_path: format!("{}/additionalProperties", schema_path),
                            keyword: "additionalProperties".to_string(),
                            message: format!("Additional property '{}' is not allowed", prop_name),
                            instance_value: Some(prop_value.clone()),
                            schema_value: Some(additional_properties.clone()),
                        });
                    }
                    Value::Object(_) => {
                        let prop_path = format!("{}/{}", instance_path, prop_name);
                        let prop_schema_path = format!("{}/additionalProperties", schema_path);
                        self.validate_recursive(prop_value, additional_properties, &prop_path, &prop_schema_path, errors);
                    }
                    _ => {} // true or non-boolean allows all additional properties
                }
            }
        }
    }

    fn validate_enum(
        &self,
        instance: &Value,
        enum_values: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if let Some(enum_array) = enum_values.as_array() {
            if !enum_array.contains(instance) {
                errors.push(ValidationError {
                    instance_path: instance_path.to_string(),
                    schema_path: format!("{}/enum", schema_path),
                    keyword: "enum".to_string(),
                    message: format!("Value is not one of the allowed enum values"),
                    instance_value: Some(instance.clone()),
                    schema_value: Some(enum_values.clone()),
                });
            }
        }
    }

    fn validate_const(
        &self,
        instance: &Value,
        const_value: &Value,
        instance_path: &str,
        schema_path: &str,
        errors: &mut Vec<ValidationError>,
    ) {
        if instance != const_value {
            errors.push(ValidationError {
                instance_path: instance_path.to_string(),
                schema_path: format!("{}/const", schema_path),
                keyword: "const".to_string(),
                message: "Value does not match the required constant".to_string(),
                instance_value: Some(instance.clone()),
                schema_value: Some(const_value.clone()),
            });
        }
    }
}

fn get_json_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

// Format validation functions
fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

fn validate_uri(uri: &str) -> bool {
    url::Url::parse(uri).is_ok()
}

fn validate_date(date: &str) -> bool {
    let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    date_regex.is_match(date)
}

fn validate_datetime(datetime: &str) -> bool {
    let datetime_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d{3})?Z?$").unwrap();
    datetime_regex.is_match(datetime)
}

fn validate_ipv4(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    parts.iter().all(|part| {
        part.parse::<u8>().is_ok()
    })
}

fn validate_ipv6(ip: &str) -> bool {
    let ipv6_regex = Regex::new(r"^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$").unwrap();
    ipv6_regex.is_match(ip)
}

fn validate_uuid(uuid: &str) -> bool {
    let uuid_regex = Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$").unwrap();
    uuid_regex.is_match(uuid)
}

// C FFI exports
#[no_mangle]
pub extern "C" fn validate_json_simple(
    schema_json: *const c_char,
    instance_json: *const c_char,
) -> *mut c_char {
    if schema_json.is_null() || instance_json.is_null() {
        return std::ptr::null_mut();
    }

    let schema_str = unsafe { CStr::from_ptr(schema_json) };
    let instance_str = unsafe { CStr::from_ptr(instance_json) };

    let schema_str = match schema_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let instance_str = match instance_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let schema: Value = match serde_json::from_str(schema_str) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let instance: Value = match serde_json::from_str(instance_str) {
        Ok(i) => i,
        Err(_) => return std::ptr::null_mut(),
    };

    let options = ValidationOptions::default();
    let validator = match JsonSchemaValidator::new(schema, options) {
        Ok(v) => v,
        Err(_) => return std::ptr::null_mut(),
    };

    let errors = validator.validate(&instance);
    let result = serde_json::to_string(&errors).unwrap_or_else(|_| "[]".to_string());

    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

// WebAssembly exports
#[wasm_bindgen]
pub fn wasm_validate_json(schema_json: &str, instance_json: &str) -> String {
    let schema: Value = match serde_json::from_str(schema_json) {
        Ok(s) => s,
        Err(e) => return format!("{{\"error\": \"Invalid schema JSON: {}\"}}", e),
    };

    let instance: Value = match serde_json::from_str(instance_json) {
        Ok(i) => i,
        Err(e) => return format!("{{\"error\": \"Invalid instance JSON: {}\"}}", e),
    };

    let options = ValidationOptions::default();
    let validator = match JsonSchemaValidator::new(schema, options) {
        Ok(v) => v,
        Err(e) => return serde_json::to_string(&e).unwrap_or_else(|_| "{}".to_string()),
    };

    let errors = validator.validate(&instance);
    serde_json::to_string(&errors).unwrap_or_else(|_| "[]".to_string())
}

#[wasm_bindgen]
pub fn wasm_is_valid(schema_json: &str, instance_json: &str) -> bool {
    let schema: Value = match serde_json::from_str(schema_json) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let instance: Value = match serde_json::from_str(instance_json) {
        Ok(i) => i,
        Err(_) => return false,
    };

    let options = ValidationOptions::default();
    let validator = match JsonSchemaValidator::new(schema, options) {
        Ok(v) => v,
        Err(_) => return false,
    };

    validator.is_valid(&instance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_validation() {
        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer", "minimum": 0}
            },
            "required": ["name"]
        });

        let valid_instance = serde_json::json!({
            "name": "John",
            "age": 30
        });

        let invalid_instance = serde_json::json!({
            "age": -5
        });

        let options = ValidationOptions::default();
        let validator = JsonSchemaValidator::new(schema, options).unwrap();

        assert!(validator.is_valid(&valid_instance));
        assert!(!validator.is_valid(&invalid_instance));

        let errors = validator.validate(&invalid_instance);
        assert_eq!(errors.len(), 2); // Missing name + negative age
    }

    #[test]
    fn test_string_validations() {
        let schema = serde_json::json!({
            "type": "string",
            "minLength": 3,
            "maxLength": 10,
            "pattern": "^[a-z]+$"
        });

        let options = ValidationOptions::default();
        let validator = JsonSchemaValidator::new(schema, options).unwrap();

        assert!(validator.is_valid(&serde_json::json!("hello")));
        assert!(!validator.is_valid(&serde_json::json!("hi"))); // too short
        assert!(!validator.is_valid(&serde_json::json!("verylongstring"))); // too long
        assert!(!validator.is_valid(&serde_json::json!("Hello"))); // uppercase
    }

    #[test]
    fn test_array_validations() {
        let schema = serde_json::json!({
            "type": "array",
            "minItems": 1,
            "maxItems": 3,
            "uniqueItems": true,
            "items": {"type": "number"}
        });

        let options = ValidationOptions::default();
        let validator = JsonSchemaValidator::new(schema, options).unwrap();

        assert!(validator.is_valid(&serde_json::json!([1, 2, 3])));
        assert!(!validator.is_valid(&serde_json::json!([]))); // too few items
        assert!(!validator.is_valid(&serde_json::json!([1, 2, 3, 4]))); // too many items
        assert!(!validator.is_valid(&serde_json::json!([1, 1, 2]))); // not unique
        assert!(!validator.is_valid(&serde_json::json!([1, "2", 3]))); // wrong type
    }

    #[test]
    fn test_format_validation() {
        let schema = serde_json::json!({
            "type": "string",
            "format": "email"
        });

        let options = ValidationOptions::default();
        let validator = JsonSchemaValidator::new(schema, options).unwrap();

        assert!(validator.is_valid(&serde_json::json!("user@example.com")));
        assert!(!validator.is_valid(&serde_json::json!("invalid-email")));
    }

    #[test]
    fn test_enum_validation() {
        let schema = serde_json::json!({
            "enum": ["red", "green", "blue"]
        });

        let options = ValidationOptions::default();
        let validator = JsonSchemaValidator::new(schema, options).unwrap();

        assert!(validator.is_valid(&serde_json::json!("red")));
        assert!(!validator.is_valid(&serde_json::json!("yellow")));
    }
}
