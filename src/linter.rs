use tree_sitter::Node;

pub fn check_for_issues(
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
                let line = node.start_position().row + 1; // Line numbers are 0-based, so add 1
                issues.push(format!("Function named 'main' found at line {line}"));
            }
        }
    }

    if node.is_named() && node.kind() == "declaration" {
        let variable_name = node
            .child_by_field_name("declarator")
            .and_then(|n| n.utf8_text(source_code).ok());

        if let Some(name) = variable_name {
            if !is_camel_case(name) {
                let line = node.start_position().row + 1; // Line numbers are 0-based, so add 1
                issues.push(format!(
                    "Variable '{name}' is not in camel case at line {line}"
                ));
            }
        }
    }

    for child in node.children(&mut node.walk()) {
        check_for_issues(child, issues, source_code, modified_code);
    }
}

fn is_camel_case(identifier: &str) -> bool {
    let mut chars = identifier.chars();
    if let Some(first_char) = chars.next() {
        if !first_char.is_lowercase() {
            return false;
        }
    }

    let mut previous_char_was_uppercase = false;
    for c in chars {
        if c == '_' || c.is_whitespace() {
            return false;
        }
        if c.is_uppercase() {
            if previous_char_was_uppercase {
                return false;
            }
            previous_char_was_uppercase = true;
        } else {
            previous_char_was_uppercase = false;
        }
    }
    true
}
