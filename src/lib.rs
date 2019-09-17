use rand::seq::SliceRandom;
use regex::Regex;
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
    pub timeout_msgs: Vec<String>,
    pub cmds: CmdTable,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct CmdTable {
    pub teleport: String,
}

impl Blocky {
    pub fn new(config_path: &str) -> Result<Blocky, Box<dyn Error>> {
        let data = fs::read_to_string(config_path)?;
        let resp: Blocky = toml::from_str(&data)?;
        Ok(resp)
    }

    pub fn respond(&self, msg: &str) -> Result<String, String> {
        let cmd_regex = Regex::new(&self.cmd_leader)
            .or_else(|err| Err(format!("Invalid cmd_leader. Error was {}", err)))?;
        match &msg {
            // Welcome messages on login
            m if m.contains("joined the game") && !m.contains("[Server]") => {
                let line: Vec<_> = m.split_whitespace().collect();
                server_say(&self.welcome_msgs, line[0])
            }
            // Messages for when a player gets kicked by lag
            // Reduce the duplication here when possible!
            m if m.contains("Timed out") && !m.contains("[Server]") => {
                let line: Vec<_> = m.split_whitespace().collect();
                server_say(&self.timeout_msgs, line[0])
            }
            // Commands
            m if cmd_regex.is_match(m) => {
                // Use RegexSet here
                let mut regex_str = String::from(r"<(\w*)> ");
                regex_str.push_str(&self.cmd_leader);
                regex_str.push_str(&self.cmds.teleport);
                let regex = Regex::new(&regex_str).or_else(|err| {
                    Err(format!(
                        "Failed to build a valid regex from {}. Error was: {}",
                        regex_str, err
                    ))
                })?;

                if let Some(names) = regex.captures(m) {
                    if names.len() == 3 {
                        return Ok(format!(
                            "/teleport {} {}\n",
                            names.get(1).unwrap().as_str(),
                            names.get(2).unwrap().as_str()
                        ));
                    } else {
                        return Err(format!(
                            "The regex \"{}\" must capture 3 items! Captured {}",
                            regex_str,
                            names.len()
                        ));
                    }
                }
                // Make this message configurable
                Ok(format!(
                    "/say \"{}\" is not a valid command...\n",
                    m.split_whitespace()
                        .skip_while(|w| {println!("skip: {}", w); w != &self.cmd_leader.trim()})
                        .skip(1)
                        .collect::<Vec<_>>()
                        .join(" ")
                ))
            }
            _ => Ok(String::new()),
        }
    }
}

// Change this to a macro
fn server_say(responses: &Vec<String>, name: &str) -> Result<String, String> {
    let mut rng = rand::thread_rng();
    let rand_resp = &responses.choose(&mut rng);
    let rand_resp = match rand_resp {
        Some(resp) => resp,
        None => return Err(format!("This config contained no messages!")),
    };
    let template = format!("/say {}\n", rand_resp);
    rt_format!(template, name = name).or(Err(format!(
        "The pattern {:?} must contain {{name}}!",
        template
    )))
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
