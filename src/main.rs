use clap::{command, Arg};
use std::fs;
use tree_sitter::{Parser, TreeCursor};
mod linter;
mod parse;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("input")
                .short('f')
                .long("file_path")
                .required(true)
                .help("Specify the path to the file to read."),
        )
        .get_matches();

    let path = matches.get_one::<String>("input").expect("required");

    let file_content = match read_file(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {e}");
            return;
        }
    };

    let mut parser = Parser::new();
    let language = tree_sitter_cpp::language();
    parser
        .set_language(&language)
        .expect("Error loading C++ grammar");

    let tree = parser
        .parse(&file_content, None)
        .expect("Error parsing C++ code");

    let root_node = tree.root_node(); // Get the root node from the parsed tree

    let mut issues = Vec::new();
    let mut modified_code = Vec::new();

    linter::check_for_issues(
        root_node,
        &mut issues,
        file_content.as_bytes(),
        &mut modified_code,
    );

    if issues.is_empty() {
        println!("No issues found");
    } else {
        for issue in issues {
            println!("Issue: {issue}");
        }
    }

    // let (issues, modified_code) = linter::lint(&tree, file_content.as_bytes());
    // if issues.is_empty() {
    //     println!("No issues found");
    // } else {
    //     for issue in issues {
    //         println!("Issue: {issue}");
    //     }
    // }

    // // Print or save the modified code
    // println!("Modified code:\n{modified_code}");
}

fn traverse_tree(cursor: &mut TreeCursor) {
    loop {
        let node = cursor.node();
        println!("{node:?}");

        if cursor.goto_first_child() {
            continue;
        }

        if cursor.goto_next_sibling() {
            continue;
        }

        while !cursor.goto_next_sibling() {
            if !cursor.goto_parent() {
                return;
            }
        }
    }
}

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}
