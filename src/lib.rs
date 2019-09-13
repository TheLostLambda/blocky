use rand::seq::SliceRandom;
use runtime_fmt::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;

// + Command leader / name?
#[derive(Debug, PartialEq, Deserialize)]
pub struct Blocky {
    pub cmd_leader: String,
    // MAKE THESE PRIVATE LATER
    pub welcome_msgs: Vec<String>,
}

impl Blocky {
    pub fn new(config_path: &str) -> Result<Blocky, Box<dyn Error>> {
        let data = fs::read_to_string(config_path)?;
        let resp: Blocky = toml::from_str(&data)?;
        Ok(resp)
    }
    pub fn respond(&self, msg: String) -> Result<String, String> {
        let mut rng = rand::thread_rng();
        match &msg {
            // Welcome messages on login
            m if m.contains("joined the game") => {
                let line: Vec<_> = m.split_whitespace().collect();
                let rand_resp = &self.welcome_msgs.choose(&mut rng);
                let rand_resp =
                    match rand_resp {
                        Some(resp) => resp,
                        None => return Err(format!(
                            "This config contained no welcome messages! Here is the config: {:#?}",
                            self
                        )),
                    };
                let template = format!("/say {}\n", rand_resp);
                rt_format!(template, name = line[0])
                    .or_else(|_| Err(format!("The pattern {:?} must contain {{name}}!", template)))
            }
            _ => Ok(String::new()),
        }
    }
}

pub fn trim_leader(line: &str) -> String {
    let delim = "]: ";
    line.split(delim).skip(1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leaders_are_trimmed() {
        let input = r#"[09:44:52] [main/WARN]: Ambiguity between arguments [teleport, targets, location] and [teleport, targets, destination] with inputs: [0.1 -0.5 .9, 0 0 0]
[09:44:52] [Server thread/INFO]: Default game type: SURVIVAL
[09:45:11] [Server-Worker-7/INFO]: Preparing spawn area: 95%
[09:45:12] [Server thread/INFO]: Done (18.829s)! For help, type "help""#;
        let expected = vec![
            r#"Ambiguity between arguments [teleport, targets, location] and [teleport, targets, destination] with inputs: [0.1 -0.5 .9, 0 0 0]"#,
            r#"Default game type: SURVIVAL"#,
            r#"Preparing spawn area: 95%"#,
            r#"Done (18.829s)! For help, type "help""#,
            ];
        let output: Vec<_> = input.lines().map(|ln| trim_leader(ln)).collect();
        assert_eq!(output, expected);
    }
}
