use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::{exit, Command};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    shell: Option<String>,
    comment: Option<String>,
    groups: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    users: Option<HashMap<String, User>>,
}

fn add_user(username: &str, user: &User) {
    let mut args = Vec::new();

    if let Some(shell) = &user.shell {
        args.push("--shell".to_owned());
        args.push("/usr/bin/".to_owned() + shell);
    }

    if let Some(comment) = &user.comment {
        args.push("-c".to_owned());
        args.push(comment.to_owned());
    }

    if let Some(groups) = &user.groups {
        args.push("--groups".to_owned());
        args.push(groups.join(","));
    }

    args.push(username.to_owned());

    match Command::new("useradd").args(args).output() {
        Ok(output) => {
            println!("StdOut: {:?}", String::from_utf8_lossy(&output.stdout));
            println!("StdErr: {:?}", String::from_utf8_lossy(&output.stderr));
        }
        Err(err) => {
            println!("Failed to create user: {:?}", err);
        }
    }
}

fn main() {
    match env::var("USER") {
        Ok(ref val) if val == "root" => {
            println!("Running as root. At your own risk.");
        }
        _ => {
            eprintln!("Must run as root/sudo.");
            exit(1);
        }
    }

    let decoded: Config = toml::from_str(&fs::read_to_string("sample.toml").unwrap()).unwrap();
    println!("decoded: {:#?}", decoded);

    for (username, user) in decoded.users.iter().flatten() {
        println!("User {}: {:?}", username, user);
        add_user(username, user);
    }
}
