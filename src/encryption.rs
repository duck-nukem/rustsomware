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
