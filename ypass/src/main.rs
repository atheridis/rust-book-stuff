use clipboard_ext::prelude::*;
use clipboard_ext::x11_fork::ClipboardContext;
use hex;
use num_bigint::BigUint;
use ring::{digest, pbkdf2};
use rpassword;
use std::io;
use std::io::Write;
use std::num::NonZeroU32;

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
pub type Credential = [u8; CREDENTIAL_LEN];

fn main() {
    const CHARACTER_SUBSETS: [(&str, &str); 4] = [
        ("lowercase", "abcdefghijklmnopqrstuvwxyz"),
        ("uppercase", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        ("digits", "0123456789"),
        ("symbols", "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~"),
    ];

    let mut name = String::new();
    let mut site = String::new();

    print!("Name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    print!("Site: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut site)
        .expect("Failed to read line");

    let master = rpassword::read_password_from_tty(Some("Pass: ")).unwrap();

    let name = name.trim();
    let site = site.trim();
    let master = master.trim();

    let length = 16;
    let counter = 0;
    let mut extrasalt = 0;

    let mut pool_of_chars = String::new();
    for i in 0..4 {
        pool_of_chars.push_str(CHARACTER_SUBSETS[i].1)
    }

    let chars: &str = &pool_of_chars[..];

    let mut password = String::new();

    while password.len() < length - 4 {
        extrasalt += 1;
        let mut salt = format!("{:x}", extrasalt).to_string();
        salt.push_str(&site);
        salt.push_str(&name);
        salt.push_str(&format!("{:x}", counter).to_string());
        let mut entropy: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new((100_000 / (length + 4)).try_into().unwrap()).unwrap(),
            salt.as_bytes(),
            master.as_bytes(),
            &mut entropy,
        );

        // println!("{}", hex::encode(entropy));
        // println!("");

        let entropy = BigUint::parse_bytes(&hex::encode(entropy).as_bytes(), 16).unwrap();
        // println!("{}", entropy);
        // println!("");
        let choice = entropy.modpow(
            &BigUint::parse_bytes(b"1", 10).unwrap(),
            &BigUint::parse_bytes(chars.len().to_string().as_bytes(), 10).unwrap(),
        );
        let choice = usize::from_str_radix(&BigUint::to_str_radix(&choice, 10), 10).unwrap();
        password.push_str(&chars[choice..(choice + 1)]);
    }

    let mut one_char_per_rule = String::new();
    for i in 0..4 {
        let av_chars = CHARACTER_SUBSETS[i].1;

        extrasalt += 1;
        let mut salt = format!("{:x}", extrasalt).to_string();
        salt.push_str(&site);
        salt.push_str(&name);
        salt.push_str(&format!("{:x}", counter).to_string());
        let mut entropy: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new((100_000 / (length + 4)).try_into().unwrap()).unwrap(),
            salt.as_bytes(),
            master.as_bytes(),
            &mut entropy,
        );

        // println!("{}", hex::encode(entropy));
        // println!("");

        let entropy = BigUint::parse_bytes(&hex::encode(entropy).as_bytes(), 16).unwrap();
        // println!("{}", entropy);
        // println!("");
        let choice = entropy.modpow(
            &BigUint::parse_bytes(b"1", 10).unwrap(),
            &BigUint::parse_bytes(av_chars.len().to_string().as_bytes(), 10).unwrap(),
        );
        let choice = usize::from_str_radix(&BigUint::to_str_radix(&choice, 10), 10).unwrap();
        one_char_per_rule.push_str(&av_chars[choice..(choice + 1)]);
    }

    for c in one_char_per_rule.chars() {
        extrasalt += 1;
        let mut salt = format!("{:x}", extrasalt).to_string();
        salt.push_str(&site);
        salt.push_str(&name);
        salt.push_str(&format!("{:x}", counter).to_string());
        let mut entropy: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new((100_000 / (length + 4)).try_into().unwrap()).unwrap(),
            salt.as_bytes(),
            master.as_bytes(),
            &mut entropy,
        );

        // println!("{}", hex::encode(entropy));
        // println!("");

        let entropy = BigUint::parse_bytes(&hex::encode(entropy).as_bytes(), 16).unwrap();

        let choice = entropy.modpow(
            &BigUint::parse_bytes(b"1", 10).unwrap(),
            &BigUint::parse_bytes(password.len().to_string().as_bytes(), 10).unwrap(),
        );

        let choice = usize::from_str_radix(&BigUint::to_str_radix(&choice, 10), 10).unwrap();

        let p1 = String::from(&password[..choice]);
        let p2 = String::from(&password[choice..]);

        password.clear();
        password.push_str(&p1);
        password.push(c);
        password.push_str(&p2);

        // let password = String::from(password[..choice].to_string().push_str(&c.to_string()))
        //     .push_str(&password[choice..].to_string());
    }

    println!("{}", password);
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password.to_owned()).unwrap();
}

fn _other() {
    let salt = "fF34@";
    let password = "hi";
    let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(1000).unwrap(),
        salt.as_bytes(),
        password.as_bytes(),
        &mut to_store,
    );
    println!("{}", hex::encode(to_store));
    let mut s = format!("{:x}", 12).to_string();
    s.push_str("hi");
    println!("{}", s);
}
