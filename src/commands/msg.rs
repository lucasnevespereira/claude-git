use crate::{api, config::Config, git, prompt};

pub fn run(config: &Config) -> Result<(), String> {
    if !git::has_changes() {
        return Err("no changes to commit".to_string());
    }

    let diff = git::get_diff(config.max_lines)?;
    let msg = api::call(config, &diff, prompt::COMMIT_MSG)?;
    println!("{msg}");
    Ok(())
}
