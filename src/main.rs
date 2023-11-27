use dialoguer::Select;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

// start off with seeing if we need to grab a password or store one
// if we need to store one
// check to see if the password file is made if not make it
// create an object with the url as the key
// ensure that the object contains username and password with thier values
// ensure the password is encrypted.
// if we need to pull a password
// ask for the url
// use that url to grab the object from the file and show the object

#[derive(Debug)]
struct Credential {
    username: String,
    password: String,
}

fn main() {
    let choices = vec!["New Password", "Grab Password", "Exit"];

    let mut credentials = HashMap::new();

    let selection = Select::new()
        .with_prompt("Select an option.")
        .default(0)
        .items(&choices)
        .interact()
        .unwrap();

    match choices[selection] {
        "New Password" => check_for_file(&mut credentials),
        "Grab Password" => grab_user_info(),
        "Exit" => close_tool(),
        &_ => todo!(), // handle all other possible cases
    }
    println!("You chose: {}", choices[selection]);
}

fn add_user_info(credentials: &mut HashMap<String, Credential>) {
    // adding the username and password to the file
    let mut url = String::new();
    let mut username = String::new();
    let mut password: String = String::new();

    println!("Enter URL: ");
    io::stdin().read_line(&mut url).unwrap();

    println!("Enter UserName: ");
    io::stdin().read_line(&mut username).unwrap();

    println!("Enter Password: ");
    io::stdin().read_line(&mut password).unwrap();

    let encrypted_password = encrypt_str(password);

    println!("adding user info");
    credentials.insert(
        url,
        Credential {
            username,
            password: encrypted_password,
        },
    );

    println!("{:?}", credentials);
}

fn encrypt_str(password: String) -> String {
    let mc = new_magic_crypt!("magickey", 256);
    let base64 = mc.encrypt_str_to_base64(password);
    return base64;
}

fn check_for_file(credentials: &mut HashMap<String, Credential>) {
    // going to check to see if the file exist or not
    println!("Checking if password file exists...");

    if !Path::new("password_manager.txt").exists() {
        println!("file does not exist... creating file now...");
        create_file();
    }

    add_user_info(credentials);
    write_obj_to_file(credentials);
}

fn write_obj_to_file(credentials: &mut HashMap<String, Credential>) {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open("password_manager.txt")
        .unwrap();

    writeln!(&mut file, "{:?}", credentials).unwrap();
}

fn grab_user_info() {
    println!("grabbing user information")
}

fn close_tool() {
    println!("closing file")
}

fn create_file() {
    // return a file or add an expect in the event file couldn't be created.
    let _ = File::create("password_manager.txt");
}
