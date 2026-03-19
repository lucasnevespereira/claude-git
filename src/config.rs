use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const DEFAULT_MODEL: &str = "haiku";
const DEFAULT_MAX_LINES: usize = 2000;
const DEFAULT_MODE: &str = "auto";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_max_lines")]
    pub max_lines: usize,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default = "default_mode")]
    pub mode: String,
}

fn default_model() -> String {
    DEFAULT_MODEL.to_string()
}
fn default_max_lines() -> usize {
    DEFAULT_MAX_LINES
}
fn default_mode() -> String {
    DEFAULT_MODE.to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: default_model(),
            max_lines: default_max_lines(),
            api_key: None,
            mode: default_mode(),
        }
    }
}

impl Config {
    pub fn path() -> PathBuf {
        dirs::home_dir()
            .expect("could not find home directory")
            .join(".claude-git.toml")
    }

    pub fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(Self::path(), content).map_err(|e| e.to_string())
    }

    pub fn get(&self, key: &str) -> Result<String, String> {
        match key {
            "model" => Ok(self.model.clone()),
            "max_lines" => Ok(self.max_lines.to_string()),
            "api_key" => Ok(self.api_key.as_ref().map(|_| "(set)".to_string()).unwrap_or("(not set)".to_string())),
            "mode" => Ok(self.mode.clone()),
            _ => Err(format!("unknown key: {key} (available: model, max_lines, api_key, mode)")),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "model" => self.model = value.to_string(),
            "max_lines" => {
                self.max_lines = value.parse().map_err(|_| "max_lines must be a number")?;
            }
            "api_key" => self.api_key = Some(value.to_string()),
            "mode" => {
                if !["auto", "api", "cli"].contains(&value) {
                    return Err("mode must be: auto, api, or cli".to_string());
                }
                self.mode = value.to_string();
            }
            _ => return Err(format!("unknown key: {key} (available: model, max_lines, api_key, mode)")),
        }
        self.save()
    }

    pub fn use_api(&self) -> bool {
        self.mode != "cli" && self.api_key.is_some()
    }

    pub fn resolve_model(&self) -> &str {
        match self.model.as_str() {
            "haiku" => "claude-haiku-4-5-20251001",
            "sonnet" => "claude-sonnet-4-6",
            "opus" => "claude-opus-4-6",
            _ => &self.model,
        }
    }

    pub fn display(&self) {
        println!("model     = {}", self.model);
        println!("max_lines = {}", self.max_lines);
        println!(
            "api_key   = {}",
            self.api_key.as_ref().map(|_| "(set)").unwrap_or("(not set)")
        );
        println!("mode      = {}", self.mode);
        println!();
        let path = Self::path();
        if path.exists() {
            println!("config: {}", path.display());
        } else {
            println!("config: (defaults, no config file)");
        }
    }
}
