use blocky::{Blocky, CmdTable};
use std::fs::{self, File};
use std::io::prelude::*;

const RESPONSE_FILE: &str = "blocky_test.toml";

const DATA: &[u8] = br#"
cmd_leader = "Blocky, "
welcome_msgs = [ "Welcome to the server, {name}!" ]
timeout_msgs = [ "Splish splash, {name}'s Internet is trash." ]
[cmds]
teleport = "take me to (\\w*)"
"#;

fn setup_file_tests() {
    let mut fh = File::create(RESPONSE_FILE).unwrap_or_else(|err| {
        panic!("Failed to create {} with error: {}", RESPONSE_FILE, err);
    });
    if let Err(err) = fh.write_all(DATA) {
        panic!(
            "Failed to write data to {} with error: {}",
            RESPONSE_FILE, err
        );
    }
}

fn cleanup_file_tests() {
    if let Err(err) = fs::remove_file(RESPONSE_FILE) {
        panic!("Failed to delete {} with error: {}", RESPONSE_FILE, err);
    }
}

#[test]
fn loads_toml() {
    setup_file_tests();
    let resp = Blocky::new(RESPONSE_FILE).unwrap_or_else(|err| {
        panic!(
            "An error occurred while parsing from {}. The error was: {}",
            RESPONSE_FILE, err
        );
    });
    cleanup_file_tests();
    let expect_resp = Blocky {
        cmd_leader: "Blocky, ".to_string(),
        welcome_msgs: vec!["Welcome to the server, {name}!".to_string()],
        timeout_msgs: vec!["Splish splash, {name}'s Internet is trash.".to_string()],
        cmds: CmdTable { teleport: r"take me to (\w*)".to_string() },
    };
    assert_eq!(resp, expect_resp);
}

#[test]
fn welcomes_user() {
    setup_file_tests();
    let block = Blocky::new(RESPONSE_FILE).unwrap_or_else(|err| {
        panic!(
            "An error occurred while parsing from {}. The error was: {}",
            RESPONSE_FILE, err
        );
    });
    let msg = "SheerFreeze joined the game";
    let resp = block.respond(msg);
    cleanup_file_tests();
    assert_eq!(
        resp,
        Ok("/say Welcome to the server, SheerFreeze!\n".to_string())
    );
}

#[test]
fn rip_user() {
    setup_file_tests();
    let block = Blocky::new(RESPONSE_FILE).unwrap_or_else(|err| {
        panic!(
            "An error occurred while parsing from {}. The error was: {}",
            RESPONSE_FILE, err
        );
    });
    let msg = "Zozzor lost connection: Timed out";
    let resp = block.respond(msg);
    cleanup_file_tests();
    assert_eq!(
        resp,
        Ok("/say Splish splash, Zozzor's Internet is trash.\n".to_string())
    );
}

#[test]
fn teleport_user() {
    setup_file_tests();
    let block = Blocky::new(RESPONSE_FILE).unwrap_or_else(|err| {
        panic!(
            "An error occurred while parsing from {}. The error was: {}",
            RESPONSE_FILE, err
        );
    });
    let msg = "<Zozzor> Blocky, take me to Sheerfreeze";
    let resp = block.respond(msg);
    cleanup_file_tests();
    assert_eq!(
        resp,
        Ok("/teleport Zozzor Sheerfreeze\n".to_string())
    );
}

#[test]
fn bad_command() {
    setup_file_tests();
    let block = Blocky::new(RESPONSE_FILE).unwrap_or_else(|err| {
        panic!(
            "An error occurred while parsing from {}. The error was: {}",
            RESPONSE_FILE, err
        );
    });
    let msg = "<Zozzor> Blocky, what is the meaning of life?";
    let resp = block.respond(msg);
    cleanup_file_tests();
    assert_eq!(
        resp,
        Ok("/say \"what is the meaning of life?\" is not a valid command...\n".to_string())
    );
}
