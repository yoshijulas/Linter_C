use tree_sitter::Node;

pub fn check_for_issues(node: Node, issues: &mut Vec<String>, source_code: &[u8]) {
    validate_node(node, source_code, issues);

    for child in node.children(&mut node.walk()) {
        check_for_issues(child, issues, source_code);
    }
}

fn validate_node(node: Node, source_code: &[u8], issues: &mut Vec<String>) {
    match node.kind() {
        "declaration" => {
            let is_const = node.children(&mut node.walk()).any(|child| {
                child.kind() == "type_qualifier" && child.utf8_text(source_code).unwrap() == "const"
            });

            check_for_short_variable_name(node, source_code, issues);

            if is_const {
                validate_constant_uppercase_name(node, source_code, issues);
            } else {
                validate_variable_camel_case_name(node, source_code, issues);
            }
        }
        "using_declaration" => {
            if node
                .utf8_text(source_code)
                .unwrap()
                .contains("namespace std")
            {
                issues.push(format!(
                    "Using namespace std is not recommended: line {}",
                    node.start_position().row + 1
                ));
            }
        }
        "for_statement" | "if_statement" | "while_statement" | "for_range_loop" => {
            if let Ok(for_text) = node.utf8_text(source_code) {
                detect_missing_space_before_control_flow_parenthesis(for_text, issues, node);
                if node.kind() == "for_statement" {
                    suggest_range_based_for_loop(for_text, issues, node);
                }
            }
        }
        "call_expression" => {
            if let Ok(call_text) = node.utf8_text(source_code) {
                if call_text.contains(" (") {
                    issues.push(format!(
                        "Function call should not have a space before '(': line {}",
                        node.start_position().row + 1
                    ));
                }
            }
        }
        "preproc_include" => {
            if node
                .utf8_text(source_code)
                .unwrap()
                .contains("<bits/stdc++.h>")
            {
                issues.push(format!(
                    "#include <bits/stdc++.h> should not be used in real programs: line {}",
                    node.start_position().row + 1
                ));
            }
        }

        _ => {}
    }
}

fn check_for_short_variable_name(node: Node, source_code: &[u8], issues: &mut Vec<String>) {
    for child in node.children(&mut node.walk()) {
        if child.kind() == "init_declarator" && child.child(0).unwrap().kind() == "identifier" {
            if let Ok(variable_name) = child.child(0).unwrap().utf8_text(source_code) {
                if variable_name.len() < 3
                    && child.parent().unwrap().parent().unwrap().kind() != "for_statement"
                {
                    let line = node.start_position().row + 1;
                    issues.push(format!(
                        "Variable '{variable_name}' is too short: line {line}"
                    ));
                }
            }
        }
    }
}

fn suggest_range_based_for_loop(for_text: &str, issues: &mut Vec<String>, node: Node) {
    if for_text.contains("for (") && !for_text.contains(':') {
        issues.push(format!(
            "Consider using range-based for loop: line {}",
            node.start_position().row + 1
        ));
    }
}

fn detect_missing_space_before_control_flow_parenthesis(
    for_text: &str,
    issues: &mut Vec<String>,
    node: Node,
) {
    if !["for (", "if (", "while ("]
        .iter()
        .any(|&s| for_text.contains(s))
    {
        issues.push(format!(
            "Control flow statement should have a space before '(': line {}",
            node.start_position().row + 1
        ));
    }
}

fn validate_variable_camel_case_name(node: Node, source_code: &[u8], issues: &mut Vec<String>) {
    for child in node.children(&mut node.walk()) {
        if child.kind() == "init_declarator" && child.child(0).unwrap().kind() == "identifier" {
            let variable_name = child.child(0).unwrap().utf8_text(source_code).ok();

            if let Some(name) = variable_name {
                if !is_camel_case(name) {
                    let line = node.start_position().row + 1;
                    issues.push(format!(
                        "Variable '{name}' is not in camel case: line {line}"
                    ));
                }
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

fn validate_constant_uppercase_name(node: Node, source_code: &[u8], issues: &mut Vec<String>) {
    for child in node.children(&mut node.walk()) {
        if child.kind() == "init_declarator" && child.child(0).unwrap().kind() == "identifier" {
            let constant_name = child.child(0).unwrap().utf8_text(source_code).ok();

            if let Some(name) = constant_name {
                if !name.chars().all(char::is_uppercase) {
                    let line = node.start_position().row + 1;
                    issues.push(format!(
                        "Const '{name}' is not all in uppercase: line {line}"
                    ));
                }
            }
        }
    }
}
