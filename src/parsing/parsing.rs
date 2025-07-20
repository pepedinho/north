use regex::Regex;

pub fn extract_install_section(readme: &str) -> Option<String> {
    let re = Regex::new(r"(?i)##?\s*(installation|install:|quick\sinstall)").unwrap();
    let mut found = false;
    let mut section = String::new();

    for line in readme.lines() {
        if re.is_match(line) {
            found = true;
        } else if found && line.starts_with('#') {
            break;
        } else if found {
            section.push_str(line);
            section.push('\n');
        }
    }
    if found {
        extract_install_cmd(&section.trim().to_string())
    } else {
        None
    }
}

pub fn extract_install_cmd(section: &str) -> Option<String> {
    let re = Regex::new(r"```([^`]+)```").unwrap();
    if let Some(captures) = re.captures(section) {
        Some(captures.get(1)?.as_str().trim().to_string())
    } else {
        None
    }
}
