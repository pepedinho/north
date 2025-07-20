use regex::Regex;

pub fn extract_install_section(readme: &str) -> Result<String, String> {
    let re = Regex::new(r"(?i)##?\s*(installation|install:|quick\sinstall|Build\s&\sRun)")
        .map_err(|e| format!("Invalid regex for section extraction: {}", e))?;
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

    if section.trim().is_empty() {
        return Err(String::from("No installation section found"));
    }
    extract_install_cmd(section.trim())
}

pub fn extract_install_cmd(section: &str) -> Result<String, String> {
    let re = Regex::new(r"```([^`]+)```").map_err(|e| format!("Invalid regex for command extraction: {}", e))?;
    if let Some(captures) = re.captures(section) {
        Ok(captures
            .get(1)
            .ok_or_else(|| String::from("No command captured"))?
            .as_str()
            .trim()
            .to_string()
        )
    } else {
        Err(String::from("Failed to extract install commands"))
    }
}
