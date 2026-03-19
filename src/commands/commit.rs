use crate::{api, config::Config, git, prompt};
use std::io::{self, Write};

pub fn run(config: &Config, auto_confirm: bool) -> Result<(), String> {
    if !git::has_changes() {
        return Err("no changes to commit".to_string());
    }

    let diff = git::get_diff(config.max_lines)?;
    let msg = api::call(config, &diff, prompt::COMMIT_MSG)?;

    println!("\n  {msg}\n");

    if auto_confirm {
        git::commit(&msg)
    } else {
        print!("Commit with this message? [y/N] ");
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;

        if input.trim() == "y" {
            git::commit(&msg)
        } else {
            println!("Aborted.");
            Ok(())
        }
    }
}
