use std::io;
use std::process::{Command, Output};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    shell: Option<String>,
    comment: Option<String>,
    groups: Option<Vec<String>>,
}

pub fn add_user(username: &str, user: &User) -> Result<Output, io::Error> {
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

    Command::new("useradd").args(args).output()
}
