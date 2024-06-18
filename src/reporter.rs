pub fn report_issues(issues: Vec<String>) {
    if issues.is_empty() {
        println!("No issues found");
    } else {
        for issue in issues {
            println!("Issue: {issue}");
        }
    }
}
