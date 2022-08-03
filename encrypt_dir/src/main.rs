extern crate aesstream;
extern crate crypto;

use aesstream::AesWriter;
use crypto::aessafe::AesSafe256Encryptor;
use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

fn is_dir_or_encrypted(entry: &DirEntry) -> bool {
    let re = Regex::new(r"\.enc$").unwrap();
    entry.file_type().is_dir() || re.is_match(entry.file_name().to_str().unwrap())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: encrypt_dir <PASSWORD> <DIRECTORY>");
    }

    let text = String::from(&args[1]);
    let digest = md5::compute(text);
    let md5 = format!("{:x}", digest);
    let password: [u8; 32] = md5.as_bytes().try_into().unwrap();

    let src_dir = &args[2];

    for entry in WalkDir::new(src_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !is_dir_or_encrypted(e))
    {
        let path_display = entry.path().display();
        println!("Reading file: {}", path_display);

        let mut input_file = match File::open(entry.path()) {
            Err(why) => {
                println!("couldn't read from {}: {}", path_display, why);
                continue;
            }
            Ok(input_file) => input_file,
        };

        let metadata = match fs::metadata(entry.path()) {
            Err(why) => {
                println!("unable to read metadata from {}: {}", path_display, why);
                continue;
            }
            Ok(metadata) => metadata,
        };

        let mut buffer = vec![0; metadata.len() as usize];
        match input_file.read(&mut buffer) {
            Err(why) => {
                println!("unable to read data from {}: {}", path_display, why);
                continue;
            }
            Ok(buffer) => buffer,
        };

        let mut enc = Vec::new();
        {
            let encryptor = AesSafe256Encryptor::new(&password);
            let mut writer = match AesWriter::new(&mut enc, encryptor) {
                Err(why) => {
                    println!("couldn't encrypt {}: {}", path_display, why);
                    continue;
                }
                Ok(writer) => writer,
            };

            match writer.write_all(&mut buffer) {
                Err(why) => {
                    println!(
                        "couldn't write the encrypted bytes into the buffer for {}: {}",
                        path_display, why
                    );
                    continue;
                }
                Ok(_) => (()),
            };
        }

        let new_path = format!("{}.enc", entry.path().as_os_str().to_str().unwrap());
        let encrypted_path = Path::new(&new_path);
        let encrypted_path_display = encrypted_path.display();
        println!("Encrypting file: {}", encrypted_path_display);

        let mut output_file = match File::create(&encrypted_path) {
            Err(why) => {
                println!("couldn't create {}: {}", encrypted_path_display, why);
                continue;
            }
            Ok(output_file) => output_file,
        };

        match output_file.write_all(enc.as_slice()) {
            Err(why) => {
                println!("couldn't write to {}: {}", encrypted_path_display, why);
                continue;
            }
            Ok(_) => println!("successfully wrote to {}", encrypted_path_display),
        };

        fs::remove_file(entry.path()).expect("File delete failed");
    }
}
