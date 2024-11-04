mod cli;
mod file_ops;

use std::error::Error;
use dialoguer::MultiSelect;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse arguments and check flags
    let (pattern, skip_errors) = cli::parse_args();

    // Retrieve matching files based on the regex pattern (or all files if no pattern)
    let entries = file_ops::list_files(pattern.as_deref())?;
    if entries.is_empty() {
        println!("No files or folders found.");
        return Ok(());
    }

    // Prompt the user to select files
    let selections = MultiSelect::new()
        .with_prompt("Select files and folders:")
        .items(&entries)
        .interact()?;

    // Confirm deletion and delete selected files
    if selections.is_empty() {
        println!("No items were selected.");
    } else if file_ops::confirm_delete()? {
        file_ops::delete_files(selections, &entries, skip_errors)?;
    } else {
        println!("Deletion canceled.");
    }

    Ok(())
}
