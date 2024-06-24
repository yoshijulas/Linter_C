use clap::{command, Arg};
use std::fs;
mod linter;
mod parse;
mod reporter;

fn main() {
    let (Some(file_content), debug) = get_file_content() else {
        return;
    };

    let tree = parse::convert_into_tree(&file_content);

    // Get the root node from the parsed tree
    let root_node = tree.root_node();

    if debug == 1 {
        // Print the AST
        let mut cursor = root_node.walk();
        parse::traverse_tree(&mut cursor);
    }

    // Initialize the linter
    let mut issues = Vec::new();

    // Call the linter on the root node of the tree
    linter::check_for_issues(root_node, &mut issues, file_content.as_bytes());

    reporter::report_issues(issues);
}

fn get_file_content() -> (Option<String>, i32) {
    let matches = command!()
        .arg(
            Arg::new("input")
                .short('f')
                .long("file_path")
                .required(true)
                .help("Specify the path to the file to read."),
        )
        .arg(
            Arg::new("Debug")
                .short('d')
                .long("debug")
                .required(false)
                .help("Enable debug mode."),
        )
        .get_matches();
    let path = matches.get_one::<String>("input").expect("required");
    let debug = matches
        .get_one::<String>("Debug")
        .map_or(0, |s| s.parse::<i32>().unwrap_or(0));
    let file_content = match read_file(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {e}");
            return (None, 0);
        }
    };
    (Some(file_content), debug)
}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}
