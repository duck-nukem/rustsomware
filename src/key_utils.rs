use rand::{Rng, thread_rng};
use uuid::Uuid;

pub fn set_encryption_key(args: &Vec<String>) -> String {
    let key: String;

    if args.len() < 4 || args[3].clone() == "" {
        key = generate_random_string(32);
    } else {
        key = args[3].clone();
    }

    println!("Using key: {}", key.clone());
    return key;
}

pub fn generate_random_string(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = thread_rng();

    let password: String = (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    return password;
}

pub fn store_key(machine_id: &Uuid, key: &String) {
    println!("\n\n🖥 {}  🔑 {}", machine_id, key);
}