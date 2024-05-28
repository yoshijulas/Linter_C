use tree_sitter::{Node, Tree};

pub fn lint(tree: &Tree, source_code: &[u8]) -> (Vec<String>, String) {
    let mut issues = Vec::new();
    let root_node = tree.root_node();
    let mut modified_code = source_code.to_vec();

    check_for_issues(root_node, &mut issues, source_code, &mut modified_code);

    (issues, String::from_utf8(modified_code).unwrap())
}

fn check_for_issues(
    node: Node,
    issues: &mut Vec<String>,
    source_code: &[u8],
    modified_code: &mut Vec<u8>,
) {
    if node.is_named() && node.kind() == "function_definition" {
        let function_name = node
            .child_by_field_name("declarator")
            .and_then(|n| n.child_by_field_name("declarator"))
            .and_then(|n| n.utf8_text(source_code).ok());

        if let Some(name) = function_name {
            if name == "main" {
                issues.push("Function named 'main' found".to_string());
            }
        }
    }

    // Additional rule: Check for global variables
    if node.is_named() && node.kind() == "declaration" {
        let variable_name = node
            .child_by_field_name("declarator")
            .and_then(|n| n.utf8_text(source_code).ok());

        if let Some(name) = variable_name {
            if is_global_variable(node) {
                issues.push(format!("Global variable '{name}' found"));
            }
        }
    }

    // Additional rule: Check for `using namespace std`
    if node.is_named() && node.kind() == "using_declaration" {
        let namespace_name = node.utf8_text(source_code).ok();

        if let Some(name) = namespace_name {
            if name == "using namespace std;" {
                issues.push("Usage of 'using namespace std' found".to_string());
                // Remove the `using namespace std` statement
                let start_byte = node.start_byte();
                let end_byte = node.end_byte();
                modified_code.splice(start_byte..end_byte, b"".iter().copied());
            }
        }
    }

    // Additional rule: Check for usage of goto statements
    if node.is_named() && node.kind() == "goto_statement" {
        issues.push("Usage of 'goto' statement found".to_string());
    }

    for child in node.children(&mut node.walk()) {
        check_for_issues(child, issues, source_code, modified_code);
    }
}

// Helper function to determine if a node represents a global variable
fn is_global_variable(node: Node) -> bool {
    // Implement logic to determine if the node represents a global variable
    // This might involve checking the parent nodes or the scope of the variable
    true // Placeholder implementation
}
