//! kirill provides predicates for scanning JSON documents.

extern crate clean_path;
extern crate json_schema_validator_core;
extern crate lazy_static;
extern crate regex;
extern crate serde_json;
extern crate walkdir;

use std::fs;
use std::path;

lazy_static::lazy_static! {
    /// DEFAULT_JSON_FILE_PATTERNS collects patterns for identifying JSON files.
    pub static ref DEFAULT_JSON_FILE_PATTERNS: regex::Regex = regex::Regex::new(
        &[
            r".*\.eslintrc",
            r".*\.jsfmtrc",
            r".*\.jslintrc",
            r".*\.jshintrc",
            r".*\.json",
        ].join("|")
    )
    .unwrap();

    /// DEFAULT_EXCLUSION_FILE_PATTERNS collects patterns for directory or file paths to skip.
    pub static ref DEFAULT_EXCLUSION_FILE_PATTERNS: regex::Regex = regex::Regex::new(
        &[
            "build",
            "target",
            "vendor",
        ].join("|")
    )
    .unwrap();
}

/// find_json_documents recursively searches
/// the given directories and/or file root paths
/// for JSON documents.
pub fn find_json_documents(roots: Vec<&path::Path>) -> Result<Vec<String>, String> {
    let mut pth_bufs = Vec::<path::PathBuf>::new();

    for root in roots {
        match fs::metadata(root) {
            Err(e) => return Err(e.to_string()),

            Ok(metadata) => {
                if metadata.is_dir() {
                    let walker = walkdir::WalkDir::new(root);

                    for entry_result in walker {
                        if let Err(e) = entry_result {
                            return Err(e.to_string());
                        }

                        let entry: walkdir::DirEntry = entry_result.unwrap();
                        let child_pth: &path::Path = entry.path();

                        if child_pth.is_dir() || child_pth.is_symlink() {
                            continue;
                        }

                        pth_bufs.push(path::PathBuf::from(child_pth));
                    }
                } else if metadata.is_file() {
                    pth_bufs.push(path::PathBuf::from(root))
                } else {
                    return Err(format!("unknown type of path: {}", root.display()));
                }
            }
        }
    }

    let mut json_documents = Vec::<String>::new();

    for pth_buf in pth_bufs {
        let pth = pth_buf.as_path();

        match path::absolute(pth) {
            Err(e) => return Err(e.to_string()),

            Ok(pth_abs) => match pth_abs.to_str() {
                None => return Err(format!("unable to process path: {}", pth_abs.display())),

                Some(pth_abs_str) => {
                    if DEFAULT_EXCLUSION_FILE_PATTERNS.is_match(pth_abs_str) {
                        continue;
                    }

                    if DEFAULT_JSON_FILE_PATTERNS.is_match(pth_abs_str) {
                        let pth_clean_buf = clean_path::clean(pth);
                        let pth_clean = pth_clean_buf.as_path();

                        match pth_clean.to_str() {
                            None => {
                                return Err(format!(
                                    "unable to process path: {}",
                                    pth_clean.display()
                                ));
                            }

                            Some(pth_clean_str) => json_documents.push(pth_clean_str.to_string()),
                        }
                    }
                }
            },
        }
    }

    Ok(json_documents)
}

/// find_json_documents_sorted lexicographically sorts any JSON document results.
pub fn find_json_documents_sorted(roots: Vec<&path::Path>) -> Result<Vec<String>, String> {
    match find_json_documents(roots) {
        Err(e) => Err(e),

        Ok(mut json_documents) => {
            json_documents.sort();
            Ok(json_documents)
        }
    }
}

/// validate_json_file checks file paths for basic JSON validity.
///
/// Returns Some(error_message) on error.
/// Otherwise, returns None.
pub fn validate_json_file_basic(s: &str) -> Option<String> {
    let pth = path::Path::new(s);

    match fs::read_to_string(pth) {
        Err(e) => Some(e.to_string()),

        Ok(contents) => {
            if let Err(e) = serde_json::from_str::<serde_json::Value>(&contents) {
                Some(e.to_string())
            } else {
                None
            }
        }
    }
}

pub fn load_json_schema_validator(
    schema_filename: &str,
) -> Result<json_schema_validator_core::JsonSchemaValidator, String> {
    match fs::read_to_string(schema_filename) {
        Err(e) => Err(e.to_string()),

        Ok(schema_contents) => match serde_json::from_str::<serde_json::Value>(&schema_contents) {
            Err(e) => Err(e.to_string()),

            Ok(schema) => match json_schema_validator_core::JsonSchemaValidator::new(
                schema,
                json_schema_validator_core::ValidationOptions::default(),
            ) {
                Err(e) => Err(e.message),

                Ok(validator) => Ok(validator),
            },
        },
    }
}

/// validate_json_file checks file paths for basic JSON validity.
///
/// Returns Some(error_message) on error.
/// Otherwise, returns None.
pub fn validate_json_file(
    s: &str,
    validator: &json_schema_validator_core::JsonSchemaValidator,
) -> Vec<String> {
    let pth = path::Path::new(s);

    match fs::read_to_string(pth) {
        Err(e) => vec![e.to_string()],

        Ok(contents) => match serde_json::from_str::<serde_json::Value>(&contents) {
            Err(e) => vec![e.to_string()],

            Ok(v) => validator
                .validate(&v)
                .iter()
                .map(|e| e.message.clone())
                .collect(),
        },
    }
}
