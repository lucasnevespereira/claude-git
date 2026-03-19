use crate::{api, config::Config, git, prompt};

pub fn run(config: &Config) -> Result<(), String> {
    if !git::has_changes() {
        return Err("no changes to review".to_string());
    }

    let diff = git::get_diff(config.max_lines)?;
    let review = api::call(config, &diff, prompt::REVIEW)?;
    println!("{review}");
    Ok(())
}
