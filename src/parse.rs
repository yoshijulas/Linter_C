use tree_sitter::{Parser, TreeCursor};

pub fn convert_into_tree(code: &str) -> tree_sitter::Tree {
    // Start the Tree-sitter parser
    let mut parser = Parser::new();
    // Set the C++ grammar as the parser's language
    let language = tree_sitter_cpp::language();
    // Initialize the parser with the C++ grammar
    parser
        .set_language(&language)
        .expect("Error loading C++ grammar");

    // Parse the C++ code
    parser.parse(code, None).expect("Error parsing C++ code")
}

pub fn traverse_tree(cursor: &mut TreeCursor) {
    fn print_node(cursor: &mut TreeCursor, level: usize) {
        let node = cursor.node();
        println!("{:indent$}{:?}", "", node, indent = level * 2);

        if cursor.goto_first_child() {
            loop {
                print_node(cursor, level + 1);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    print_node(cursor, 0);
}
