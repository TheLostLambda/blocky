use runtime_fmt::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;

// + Command leader / name?
#[derive(Debug, PartialEq, Deserialize)]
pub struct Blocky {
    // MAKE THIS PRIVATE LATER
    pub welcome_msgs: Vec<String>,
}

impl Blocky {
    pub fn new(config_path: &str) -> Result<Blocky, Box<dyn Error>> {
        let data = fs::read_to_string(config_path)?;
        let resp: Blocky = toml::from_str(&data)?;
        Ok(resp)
    }
    pub fn respond(&self, msg: String) -> Result<String, String> {
        match &msg {
            m if m.contains("joined the game") => {
                let line: Vec<_> = m.split_whitespace().collect();
                let template = format!("/say {}\n", &self.welcome_msgs[0]);
                rt_format!(template, name = line[0])
                    .or_else(|_| Err(format!("The pattern {:?} must contain {{name}}!", template)))
            }
            _ => Ok(String::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
