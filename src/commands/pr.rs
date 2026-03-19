use crate::{api, config::Config, git, prompt};

pub fn run(config: &Config, base: &str) -> Result<(), String> {
    let diff = git::get_branch_diff(base, config.max_lines)?;
    let description = api::call(config, &diff, prompt::PR)?;
    println!("{description}");
    Ok(())
}
