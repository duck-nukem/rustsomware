use rand::{Rng, thread_rng};

pub fn set_encryption_key(args: &Vec<String>) -> String {
    let key: String;

    if args.len() < 4 || args[3].clone() == "" {
        key = generate_random_string();
    } else {
        key = args[3].clone();
    }

    println!("Using key: {}", key.clone());
    return key;
}

fn generate_random_string() -> String {
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
