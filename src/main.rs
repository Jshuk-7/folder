//! This binary crate a simple terminal tool to create and delete folders

//! # Examples
//! ```
//! folder <dir_name>
//! folder --remove <dir_name>
//! ```

use std::env;
use std::fs;
use std::io::stdin;
use std::process;
use version::version;

fn main() {
    let mut args = env::args().skip(1);

    if let Some(dir_path) = args.next() {
        match dir_path.as_str() {
            "--help" => {
                execute_command(Command::PrintHelp);
                execute_command(Command::Quit(0));
            }
            "--version" => {
                execute_command(Command::PrintVersion);
                execute_command(Command::Quit(0));
            }
            "--remove" => {
                let dir_to_be_removed = args.next().unwrap();
                let mut verification_str = String::new();
                println!("Permanently remove {dir_to_be_removed}/, cannot be undone <Y/N>:");

                if stdin().read_line(&mut verification_str).is_err() {
                    println!("Failed to read line!\nExiting program...");
                    execute_command(Command::Quit(1));
                } else if verification_str.trim() == "y" {
                    if fs::remove_dir_all(&dir_to_be_removed).is_err() {
                        println!("Failed to remove '{dir_to_be_removed}': Resource busy, Exiting program...");
                        execute_command(Command::Quit(1));
                    }

                    println!("Directory {dir_to_be_removed}/ successfully deleted!");
                    execute_command(Command::Quit(0));
                }
            }
            _ => (),
        }

        if fs::create_dir(&dir_path).is_err() {
            println!(
                r#"
Failed to create directory {dir_path}/, it may already exist!
For help type folder --help
            "#
            );

            execute_command(Command::Quit(0));
        } else if let Some(file_name) = args.next() {
            let file = format!("{}/{}", &dir_path, file_name);

            if let Some(contents) = args.next() {
                if fs::write(&file, &contents).is_err() {
                    println!("Failed to write contents to file: {file},\n{contents}");
                    execute_command(Command::Quit(1));
                }
            } else if fs::write(&file, "").is_err() {
                println!("Failed to create File: {file}");
                execute_command(Command::Quit(1));
            }

            println!("Directory {dir_path}/ successfully created with file: {file_name}");
        } else {
            println!("Directory {dir_path}/ successfully created!");
        }
    } else {
        execute_command(Command::PrintHelp);
        execute_command(Command::Quit(0));
    };
}

enum Command {
    PrintHelp,
    PrintVersion,
    Quit(i32),
}

fn execute_command(command: Command) {
    match command {
        Command::PrintHelp => print_help(),
        Command::PrintVersion => {
            println!(
                r#"
folder {},
A command line tool by Jessie 2022
            "#,
                version!()
            )
        }
        Command::Quit(code) => process::exit(code),
    }
}

fn print_help() {
    println!(
        r#"
------- HELP -------

Folder Tool Arguments:
    <dir_name>     Name for the new directory,
    <file_name>    Optional file to be created in the new directory,
    <contents>     Optional contents for the new file

Adittional Commands
    --remove       removes a directory PERMANENTLY
    --help         prints help
    --version      prints version
    "#
    );
}
