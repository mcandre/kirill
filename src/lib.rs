//! kirill provides predicates for scanning JSON documents.

extern crate clean_path;
extern crate json_schema_validator_core;
extern crate lazy_static;
extern crate regex;
extern crate serde_json;
extern crate serde_json5;
extern crate walkdir;

use std::fmt;
use std::fs;
use std::path;
use std::process;

lazy_static::lazy_static! {
    /// JSON_FILE_EXTENSIONS collects common JSON file extensions.
    pub static ref JSON_FILE_EXTENSIONS: Vec<String> = vec![
        // eslint
        "eslintrc".to_string(),

        // jsfmt
        "jsfmtrc".to_string(),

        // jslint
        "jslintrc".to_string(),

        // jshint
        "jshintrc".to_string(),

        // JSON
        "json".to_string(),
    ];

    /// JSON5_FILE_EXTENSIONS collects common JSON5 file extensions.
    pub static ref JSON5_FILE_EXTENSIONS: Vec<String> = JSON_FILE_EXTENSIONS
        .iter()
        .chain(
            &[
                // JSON5
                "json5".to_string(),
            ]
        ).cloned()
        .collect();

    /// DEFAULT_JSON_FILE_PATTERNS collects patterns for identifying JSON files.
    pub static ref DEFAULT_JSON_FILE_PATTERNS: regex::Regex = regex::Regex::new(
        &format!(
            "({})$",
            JSON_FILE_EXTENSIONS
                .iter()
                .map(|e| format!(r".*\.{}", e))
                .collect::<Vec<String>>()
                .join("|")
        )
    )
    .unwrap();

    /// DEFAULT_JSON5_FILE_PATTERNS collects patterns for identifying JSON files.
    pub static ref DEFAULT_JSON5_FILE_PATTERNS: regex::Regex = regex::Regex::new(
        &format!(
            "({})$",
            JSON5_FILE_EXTENSIONS
                .iter()
                .map(|e| format!(r".*\.{}", e))
                .collect::<Vec<String>>()
                .join("|")
        )
    )
    .unwrap();

    /// DEFAULT_EXCLUSION_FILE_PATTERNS collects patterns for directory or file paths to skip.
    pub static ref DEFAULT_EXCLUSION_FILE_PATTERNS: regex::Regex = regex::Regex::new(
        &[
            // crit
            ".crit",

            // Python
            ".venv",

            // C/C++
            "build",

            // Node.js
            "node_modules",
            "package-lock.json",

            // .NET, Go, JVM, Rust
            "target",

            // Go, Rust
            "vendor",
        ].join("|")
    )
    .unwrap();
}

/// KirillError models bad computer states.
#[derive(Debug)]
pub enum KirillError {
    IOError(String),
    DirectoryTraversalError(walkdir::Error),
    UnsupportedPathError(String),
    PathRenderError(String),
    JSONParseError(serde_json::Error),
    JSON5ParseError(serde_json5::Error),
    JSONSchemaError(String),
}

impl fmt::Display for KirillError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KirillError::IOError(e) => write!(f, "{}", e),
            KirillError::DirectoryTraversalError(e) => write!(f, "{}", e),
            KirillError::UnsupportedPathError(e) => write!(f, "{}", e),
            KirillError::PathRenderError(e) => write!(f, "{}", e),
            KirillError::JSONParseError(e) => write!(f, "{}", e),
            KirillError::JSON5ParseError(e) => write!(f, "{}", e),
            KirillError::JSONSchemaError(e) => write!(f, "{}", e),
        }
    }
}

impl die::PrintExit for KirillError {
    fn print_exit(&self) -> ! {
        eprintln!("{}", self);
        process::exit(die::DEFAULT_EXIT_CODE);
    }
}

/// find_json_documents recursively searches
/// the given directories and/or file root paths
/// for JSON documents.
pub fn find_json_documents(
    roots: Vec<&path::Path>,
    parse_json5: bool,
) -> Result<Vec<String>, KirillError> {
    let mut pth_bufs = Vec::<path::PathBuf>::new();

    for root in roots {
        let metadata = fs::metadata(root).map_err(|_| {
            KirillError::IOError(format!(
                "unable to query metadata for path: {}",
                root.display()
            ))
        })?;

        if metadata.is_dir() {
            let walker = walkdir::WalkDir::new(root);

            for entry_result in walker {
                let entry = entry_result.map_err(KirillError::DirectoryTraversalError)?;
                let child_pth: &path::Path = entry.path();

                if child_pth.is_dir() || child_pth.is_symlink() {
                    continue;
                }

                pth_bufs.push(path::PathBuf::from(child_pth));
            }
        } else if metadata.is_file() {
            pth_bufs.push(path::PathBuf::from(root))
        } else {
            return Err(KirillError::UnsupportedPathError(format!(
                "unknown type of path: {}",
                root.display()
            )));
        }
    }

    let mut json_documents = Vec::<String>::new();

    for pth_buf in pth_bufs {
        let pth = pth_buf.as_path();
        let pth_abs = path::absolute(pth).map_err(|_| {
            KirillError::IOError(format!("unable to resolve path: {}", pth.display()))
        })?;
        let pth_abs_str = pth_abs
            .to_str()
            .ok_or(KirillError::PathRenderError(format!(
                "unable to process path: {}",
                pth_abs.display()
            )))?;

        if DEFAULT_EXCLUSION_FILE_PATTERNS.is_match(pth_abs_str) {
            continue;
        }

        let pattern = if parse_json5 {
            &*DEFAULT_JSON5_FILE_PATTERNS
        } else {
            &*DEFAULT_JSON_FILE_PATTERNS
        };

        if pattern.is_match(pth_abs_str) {
            let pth_clean_buf = clean_path::clean(pth);
            let pth_clean = pth_clean_buf.as_path();
            let pth_clean_str = pth_clean
                .to_str()
                .ok_or(KirillError::PathRenderError(format!(
                    "unable to process cleaned path: {}",
                    pth_clean.display()
                )))?;
            json_documents.push(pth_clean_str.to_string())
        }
    }

    Ok(json_documents)
}

/// find_json_documents_sorted lexicographically sorts any JSON document results.
pub fn find_json_documents_sorted(
    roots: Vec<&path::Path>,
    parse_json5: bool,
) -> Result<Vec<String>, KirillError> {
    let mut json_documents = find_json_documents(roots, parse_json5)?;
    json_documents.sort();
    Ok(json_documents)
}

/// validate_json_file checks file paths for basic JSON validity.
///
/// Returns Some(error) on error.
/// Otherwise, returns None.
pub fn validate_json_file_basic(s: &str, parse_json5: bool) -> Option<KirillError> {
    let pth = path::Path::new(s);

    match fs::read_to_string(pth) {
        Err(_) => Some(KirillError::IOError(format!(
            "unable to read path: {}",
            pth.display()
        ))),

        Ok(contents) => {
            if parse_json5 {
                serde_json5::from_str::<serde_json::Value>(&contents)
                    .map_err(KirillError::JSON5ParseError)
                    .err()
            } else {
                serde_json::from_str::<serde_json::Value>(&contents)
                    .map_err(KirillError::JSONParseError)
                    .err()
            }
        }
    }
}

pub fn load_json_schema_validator(
    schema_filename: &str,
) -> Result<json_schema_validator_core::JsonSchemaValidator, KirillError> {
    let contents = fs::read_to_string(schema_filename)
        .map_err(|_| KirillError::IOError(format!("unable to read file: {}", schema_filename)))?;
    let schema = serde_json::from_str::<serde_json::Value>(&contents)
        .map_err(KirillError::JSONParseError)?;

    json_schema_validator_core::JsonSchemaValidator::new(
        schema,
        json_schema_validator_core::ValidationOptions::default(),
    )
    .map_err(|e| KirillError::JSONSchemaError(e.message))
}

/// validate_json_file checks file paths for basic JSON validity.
///
/// Returns Some(error_message) on error.
/// Otherwise, returns None.
pub fn validate_json_file(
    s: &str,
    validator: &json_schema_validator_core::JsonSchemaValidator,
) -> Result<(), Vec<KirillError>> {
    let pth = path::Path::new(s);
    let contents = fs::read_to_string(pth).map_err(|_| {
        vec![KirillError::IOError(format!(
            "unable to read file: {}",
            pth.display()
        ))]
    })?;
    let value = serde_json::from_str::<serde_json::Value>(&contents)
        .map_err(|e| vec![KirillError::JSONParseError(e)])?;

    Err(validator
        .validate(&value)
        .iter()
        .map(|e| KirillError::JSONSchemaError(e.message.clone()))
        .collect::<Vec<KirillError>>())
}
