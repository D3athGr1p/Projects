use argon2::Argon2;
use base64;
use chacha20poly1305::{
    self,
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key,
};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Seek, Write},
    process, str,
};

#[derive(Debug)]
enum Process {
    Encryption,
    Decryption,
    Help,
}

#[derive(Debug)]
pub struct Config {
    filename: Option<String>,
    operation: Process,
    password: Option<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let mut config = parse_args(&args).unwrap_or_else(|err| {
            eprintln!("Error parsing arguments: {}", err);
            print_help();
            process::exit(1);
        });

        match &config.operation {
            Process::Help => {
                print_help();
                process::exit(0)
            }
            _ => println!("..."),
        }

        if config.password.is_none() {
            let mut password = String::new();

            println!("Please enter the password: ");
            io::stdin()
                .read_line(&mut password)
                .expect("failed to read line");

            config.password = Some(password.trim().to_owned());
        }

        Ok(config)
    }
}

fn print_help() {
    println!(
        "
Usage: cargo run [OPTION]
    -h, --help      : print help message
    -e, --encrypt   : encrypt the file
    -d, --decrypt   : decrypt the file
    -p, --password  : set the password
    -f, --file      : set the file to encrypt/decrypt
    "
    );
}

fn parse_args(args: &[String]) -> Result<Config, &'static str> {
    let mut config = Config {
        operation: Process::Help,
        password: None,
        filename: None,
    };

    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => config.operation = Process::Help,
            "-e" | "--encrypt" => config.operation = Process::Encryption,
            "-d" | "--decrypt" => config.operation = Process::Decryption,
            "-p" | "--password" => {
                if i + 1 < args.len() {
                    config.password = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    return Err("Missing value for file");
                }
            }
            "-f" | "--file" => {
                if i + 1 < args.len() {
                    config.filename = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    return Err("Missing value for file");
                }
            }
            _ => return Err("Unknown argument"),
        }

        i += 1;
    }

    Ok(config)
}

fn check_file_exist(config: &Config) -> (File, File) {
    let file_path: String = config.filename.clone().unwrap();

    match &config.operation {
        Process::Encryption => {
            println!("Encrypting file {}", file_path);
            (
                OpenOptions::new()
                    .read(true)
                    .open(&file_path)
                    .expect("Couldn't open file"),
                OpenOptions::new()
                    .read(true)
                    .append(true)
                    .write(true)
                    .create(true)
                    .open(format!("{}{}", &file_path, ".encrypted"))
                    .expect("Had some issue"),
            )
        }

        Process::Decryption => {
            println!("Decrypting file {}.encrypted", file_path);
            (
                OpenOptions::new()
                    .read(true)
                    .append(true)
                    .write(true)
                    .create(true)
                    .open(format!("{}{}", &file_path, ".encrypted"))
                    .expect("Had some issue"),
                OpenOptions::new()
                    .read(true)
                    .open(&file_path)
                    .expect("Couldn't open file"),
            )
        }
        _ => unreachable!(),
    }
}

fn derive_key_from_password(password: &str) -> [u8; 32] {
    let mut hasher = DefaultHasher::new();
    Hash::hash_slice(password.as_bytes(), &mut hasher);

    let salt = [hasher.finish() as u8; 32];
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(password.as_bytes(), &salt, &mut key)
        .expect("issue during password hashing");
    key
}

fn encryption(
    config: &Config,
    read_file: &mut File,
    write_file: &mut File,
) -> Result<(), Box<dyn Error>> {
    let mut encrypted_content = String::new();
    let decrypted_content = BufReader::new(read_file);

    let key = derive_key_from_password(config.password.clone().unwrap().as_str());

    let key = Key::from_slice(&key);

    // let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);

    for line in decrypted_content.lines() {
        let line: String = line?;

        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        let ciphertext = cipher
            .encrypt(&nonce, line.as_bytes())
            .map_err(|err| format!("Encryption error: {}", err))?;

        let encoded_nonce = base64::encode(&nonce);
        let encoded_ciphertext = base64::encode(&ciphertext);

        encrypted_content.push_str(&encoded_nonce);
        encrypted_content.push(':');
        encrypted_content.push_str(&encoded_ciphertext);
        encrypted_content.push('\n');
    }

    write_file.set_len(0)?;
    write_file.seek(std::io::SeekFrom::Start(0))?;
    write_file.write_all(encrypted_content.as_bytes())?;

    Ok(())
}

fn decryption(config: &Config, mut write_file: File) -> Result<(), Box<dyn Error>> {
    let mut decrypted_content = String::new();

    let key = derive_key_from_password(config.password.clone().unwrap().as_str());

    let key = Key::from_slice(&key);

    let cipher = ChaCha20Poly1305::new(&key);

    let encrypted_content = BufReader::new(&write_file);

    for line in encrypted_content.lines() {
        let line: String = line?;
        let splits: Vec<&str> = line.split(':').collect();

        let (nonce, text) = (splits[0], splits[1]);

        let nonce = base64::decode(nonce).unwrap();

        let nonce = chacha20poly1305::Nonce::from_slice(&nonce);

        let vecs = base64::decode(&text)?;

        let plaintext = cipher
            .decrypt(&nonce, vecs.as_ref())
            .map_err(|err| format!("Decryption error: {}", err))?;

        let plaintext = str::from_utf8(&plaintext).unwrap();

        decrypted_content.push_str(plaintext);
        decrypted_content.push('\n');
    }

    write_file.set_len(0)?;
    write_file.seek(std::io::SeekFrom::Start(0))?;
    write_file.write_all(decrypted_content.as_bytes())?;

    Ok(())
}

pub fn run(config: &Config) {
    let (mut file_pointer_1, mut file_pointer_2) = check_file_exist(config);

    match config.operation {
        Process::Encryption => encryption(config, &mut file_pointer_1, &mut file_pointer_2),
        Process::Decryption => decryption(config, file_pointer_1),
        Process::Help => unreachable!(),
    }
    .unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        process::exit(1);
    });

    println!("Done");
}
