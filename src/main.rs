use dialoguer::Select;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::io;

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
        &_ => todo!() // handle all other possible cases
    }
    println!("You chose: {}", choices[selection]);
}

fn add_user_info(credentials: &mut HashMap<String, Credential>) {
    // adding the username and password to the file
    let mut url = String::new();
    let mut username = String::new();
    let mut password = String::new();

    println!("Enter URL: ");
    io::stdin().read_line(&mut url).unwrap();

    println!("Enter UserName: ");
    io::stdin().read_line(&mut username).unwrap();

    println!("Enter Password: ");
    io::stdin().read_line(&mut password).unwrap();

    println!("adding user info");
    credentials.insert(url, Credential {username, password});

    println!("{:?}", credentials);
}

fn check_for_file(credentials: &mut HashMap<String, Credential>) {
    // going to check to see if the file exist or not
    println!("Checking if password file exists...");

    if !Path::new("password_manager.txt").exists() {
        println!("file does not exist... creating file now...");
        create_file();
    }
    
    add_user_info(credentials)
}

fn grab_user_info() {
    println!("grabbing user information")
}

fn close_tool() {
    println!("closing file")
}

fn create_file() {
    File::create("password_manager.txt");
}


