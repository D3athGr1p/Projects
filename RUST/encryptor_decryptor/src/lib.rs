use chacha20poly1305::{
    self,
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
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
    let file_path = config.filename.clone().unwrap();

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
                    .open(format!("{}{}", &file_path, ".encrypted.txt"))
                    .expect("Had some issue"),
            )
        }

        Process::Decryption => {
            println!("Decrypting file {}", file_path);
            (
                OpenOptions::new()
                    .read(true)
                    .append(true)
                    .write(true)
                    .create(true)
                    .open(format!("{}{}", &file_path, ".encrypted.txt"))
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

fn encryption(
    config: &Config,
    read_file: &mut File,
    write_file: &mut File,
) -> Result<(), Box<dyn Error>> {
    let mut encrypted_content = String::new();
    let decrypted_content = BufReader::new(read_file);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = ChaCha20Poly1305::new(&key);
    let mut encrypted = vec![0; 16];

    for line in decrypted_content.lines() {
        let line: String = line?;

        let ciphertext = cipher.encrypt(&nonce, line.as_ref());
        encrypted_content.push_str(ciphertext);
        encrypted_content.push('\n');
    }

    write_file.set_len(0)?;
    write_file.seek(std::io::SeekFrom::Start(0))?;
    write_file.write_all(encrypted_content.as_bytes())?;

    Ok(())
}

fn decryption(config: &Config, read_file: &File, write_file: &File) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn run(config: &Config) {
    let (mut file_pointer_1, mut file_pointer_2) = check_file_exist(config);

    let _ = match config.operation {
        Process::Encryption => encryption(config, &mut file_pointer_1, &mut file_pointer_2),
        Process::Decryption => decryption(config, &file_pointer_2, &file_pointer_2),
        Process::Help => unreachable!(),
    };
}
