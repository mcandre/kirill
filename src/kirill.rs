//! CLI kirill tool

extern crate die;
extern crate getopts;
extern crate kirill;

use die::{Die, die};
use std::env;
use std::path;
use std::process;

/// CLI entrypoint
fn main() {
    let brief: String = format!(
        "usage: {} [OPTIONS] <path> [<path> [<path> ...]]",
        env!("CARGO_PKG_NAME")
    );

    let mut opts: getopts::Options = getopts::Options::new();
    opts.optflag("5", "json5", "parse according to JSON5");
    opts.optflag("l", "list", "list JSON files");
    opts.optopt(
        "s",
        "schema",
        "validate files against a given JSON Schema file",
        "<path>",
    );
    opts.optflag("h", "help", "print usage info");
    opts.optflag("v", "version", "print version info");

    let usage: String = opts.usage(&brief);
    let arguments: Vec<String> = env::args().collect();
    let optmatches: getopts::Matches = opts.parse(&arguments[1..]).die(&usage);

    if optmatches.opt_present("h") {
        die!(0; usage);
    }

    if optmatches.opt_present("v") {
        die!(0; format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
    }

    let parse_json5: bool = optmatches.opt_present("5");
    let list_documents: bool = optmatches.opt_present("l");
    let validate_schema: bool = optmatches.opt_present("s");

    if parse_json5 && validate_schema {
        eprintln!("error: JSON5 incompatible with JSON Schema");
        die!(usage);
    }

    let schema_option: Option<String> = optmatches.opt_str("s");

    if validate_schema && schema_option.is_none() {
        die!(usage);
    }

    let rest_args = optmatches.free;

    if rest_args.is_empty() {
        die!(usage);
    }

    let roots: Vec<&path::Path> = rest_args.iter().map(path::Path::new).collect();

    let json_documents_result = kirill::find_json_documents_sorted(roots, parse_json5);

    if let Err(e) = json_documents_result {
        die!(e);
    }

    let json_documents = json_documents_result.unwrap();

    if list_documents {
        for json_document in json_documents {
            println!("{}", json_document);
        }
    } else if validate_schema {
        let schema_filename = schema_option.unwrap();

        match kirill::load_json_schema_validator(&schema_filename) {
            Err(e) => die!("error: {}: {}", schema_filename, e),

            Ok(validator) => {
                for json_document in json_documents {
                    let errors =
                        kirill::validate_json_file(&json_document, &validator).unwrap_err();

                    if !errors.is_empty() {
                        for e in errors {
                            eprintln!("error: {}: {}", json_document, e);
                        }

                        process::exit(die::DEFAULT_EXIT_CODE);
                    }
                }
            }
        }
    } else {
        let mut found_invalid: bool = false;

        for json_document in json_documents {
            if let Some(e) = kirill::validate_json_file_basic(&json_document, parse_json5) {
                found_invalid = true;
                eprintln!("error: {}: {}", json_document, e);
            }
        }

        if found_invalid {
            process::exit(die::DEFAULT_EXIT_CODE);
        }
    }
}
