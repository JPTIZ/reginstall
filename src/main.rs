use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::process::{exit, Command, Output};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    shell: Option<String>,
    comment: Option<String>,
    groups: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PackageList {
    flags: Option<String>,
    pkglist: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    users: Option<HashMap<String, User>>,
    packages: Option<HashMap<String, PackageList>>,
}

fn add_user(username: &str, user: &User) -> Result<Output, io::Error> {
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

fn install_packages(manager: &str, packages: &PackageList) -> Result<Output, io::Error> {
    let flags = match &packages.flags {
        Some(f) => f.split(" ").collect::<Vec<_>>(),
        None => vec![],
    };

    let default = vec![];

    let pkglist = *(&packages.pkglist.as_ref().unwrap_or(&default));

    Command::new(manager).args(flags).args(pkglist).output()
}

fn run_or_die(result: Result<Output, io::Error>) {
    match result {
        Ok(output) => {
            if !&output.stderr.is_empty() {
                println!(" Error: {}", String::from_utf8_lossy(&output.stderr).trim());
                return;
            }
            println!(
                "Done. Output:\n{:?}",
                String::from_utf8_lossy(&output.stdout)
            );
        }
        Err(err) => {
            println!("Failed: {:?}", err);
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
        print!("Creating user {}...", username);
        run_or_die(add_user(username, user));
    }

    for (manager, packages) in decoded.packages.iter().flatten() {
        println!("Package manager {}: {:?}", manager, packages);
        run_or_die(install_packages(manager, packages));
    }
}
