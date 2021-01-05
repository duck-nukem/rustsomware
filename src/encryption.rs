use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use enc_file::{decrypt_chacha, encrypt_chacha};

pub fn encrypt_file(original_file: &mut File, key: &String) {
    let mut content = vec![];
    original_file.read_to_end(&mut content).unwrap();

    let encrypted_content = encrypt_chacha(content, key).unwrap();

    original_file.set_len(0).unwrap();
    original_file.seek(SeekFrom::Start(0)).unwrap();
    original_file.write_all(encrypted_content.as_slice()).unwrap();
    print!(" ...done!");
}

pub fn decrypt_file(encrypted_file: &mut File, key: &String) {
    let mut encrypted_content = vec![];
    encrypted_file.read_to_end(&mut encrypted_content).unwrap();

    let original_content = decrypt_chacha(encrypted_content, key).unwrap();
    encrypted_file.set_len(0).unwrap();
    encrypted_file.seek(SeekFrom::Start(0)).unwrap();
    encrypted_file.write_all(original_content.as_slice()).unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs::{File, OpenOptions};
    use std::io::{Write, Read};
    use crate::encryption::{encrypt_file, decrypt_file};

    #[test]
    fn encrypts_file() {
        let key = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let mut file = File::create("encrypt.txt").unwrap();
        let original_content = "hello";
        file.write_all(original_content.as_bytes()).unwrap();
        file = open_file("encrypt.txt");

        encrypt_file(&mut file, &key.to_owned());

        let mut contents = String::new();
        file = open_file("encrypt.txt");

        // Expect to get errors reading as the encrypted file won't be valid UTF-8
        match file.read_to_string(&mut contents) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    fn open_file(filename: &str) -> File {
        return OpenOptions::new().read(true).write(true).open(filename).unwrap();
    }

    #[test]
    fn decrypts_file() {
        let key = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let mut file = File::create("encrypt.txt").unwrap();
        let original_content = "hello";
        file.write_all(original_content.as_bytes()).unwrap();

        file = open_file("encrypt.txt");
        encrypt_file(&mut file, &key.to_owned());

        file = open_file("encrypt.txt");
        decrypt_file(&mut file, &key.to_owned());

        let mut contents = String::new();
        file = open_file("encrypt.txt");
        file.read_to_string(&mut contents).unwrap();

        assert_eq!(contents, original_content);
    }
}