use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::process::{exit, Command, Output};

use serde::{Deserialize, Serialize};
use clap::{App, SubCommand};

mod users;
use users::*;

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
    let matches = App::new("reginstall")
                    .version("0.2.0")
                    .author("João Paulo Taylor Ienczak Zanette <jpaulotiz@gmail.com>")
                    .about("Manages basic setup for easier machine switching.")
                    .subcommand(SubCommand::with_name("setup")
                            .about("Makes full setup from reginstall config file.")
                            .arg_from_usage("<FILE> 'reginstall config file'."))
                    .get_matches();

    if let Some(matches) = matches.subcommand_matches("setup") {
        println!("Running setup command with file \"{}\"", matches.value_of("FILE").unwrap());

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
    } else {
        println!("Subcommand needed!");
    }
}
