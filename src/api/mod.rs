pub mod anthropic;
pub mod claude_cli;

use crate::config::Config;

pub fn call(config: &Config, input: &str, prompt: &str) -> Result<String, String> {
    if config.use_api() {
        let api_key = config.api_key.as_ref().unwrap();
        let model = config.resolve_model();
        anthropic::call(api_key, model, input, prompt)
    } else {
        claude_cli::call(&config.model, input, prompt)
    }
}
