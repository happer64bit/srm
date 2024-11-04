use std::error::Error;
use std::fs;
use regex::Regex;
use dialoguer::Confirm;

// List files in the current directory that match the provided regex pattern (or all files if no pattern)
pub fn list_files(pattern: Option<&str>) -> Result<Vec<String>, Box<dyn Error>> {
    // If a pattern is provided, compile the regex
    let regex = pattern.map(|p| Regex::new(p)).transpose()?;

    let entries: Vec<String> = fs::read_dir(".")?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().into_string().unwrap_or_default())
        .filter(|name| match &regex {
            Some(re) => re.is_match(name), // Apply regex filter if pattern exists
            None => true,                  // Include all files if no pattern
        })
        .collect();

    Ok(entries)
}

// Prompt the user to confirm deletion of selected files
pub fn confirm_delete() -> Result<bool, Box<dyn Error>> {
    Ok(Confirm::new()
        .with_prompt("Do you want to delete the selected items?")
        .interact()?)
}

// Delete selected files with optional error skipping
pub fn delete_files(
    selections: Vec<usize>,
    entries: &Vec<String>,
    skip_errors: bool,
) -> Result<(), Box<dyn Error>> {
    for index in selections {
        let file_name = &entries[index];
        if let Err(e) = fs::remove_file(file_name) {
            if skip_errors {
                eprintln!("Failed to delete '{}', but skipping due to -s: {}", file_name, e);
            } else {
                eprintln!("Failed to delete '{}': {}", file_name, e);
                return Err(Box::new(e));
            }
        } else {
            println!("Deleted '{}'", file_name);
        }
    }
    Ok(())
}
