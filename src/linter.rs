use tree_sitter::Node;

pub fn check_for_issues(
    node: Node,
    issues: &mut Vec<String>,
    source_code: &[u8],
    modified_code: &mut Vec<u8>,
) {
    validate_variable_name(node, source_code, issues);

    for child in node.children(&mut node.walk()) {
        check_for_issues(child, issues, source_code, modified_code);
    }
}

fn validate_variable_name(node: Node, source_code: &[u8], issues: &mut Vec<String>) {
    if node.is_named() && node.kind() == "identifier" {
        let variable_name = node.utf8_text(source_code).ok();

        if let Some(name) = variable_name {
            if !is_camel_case(name) {
                let line = node.start_position().row + 1; // Line numbers are 0-based, so add 1
                issues.push(format!(
                    "Variable '{name}' is not in camel case at line {line}"
                ));
            }
        }
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
        match c {
            '_' | ' ' => return false,
            _ if c.is_uppercase() => {
                if previous_char_was_uppercase {
                    return false;
                }
                previous_char_was_uppercase = true;
            }
            _ => previous_char_was_uppercase = false,
        }
    }
    true
}
