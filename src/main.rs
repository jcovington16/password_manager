use dialoguer::Select;

// start off with seeing if we need to grab a password or store one
// if we need to store one
    // check to see if the password file is made if not make it
    // create an object with the url as the key
        // ensure that the object contains username and password with thier values
            // ensure the password is encrypted.
// if we need to pull a password
    // ask for the url
        // use that url to grab the object from the file and show the object


fn main() {
    let choices = vec!["New Password", "Grab Password", "Exit"];
    
    let selection = Select::new()
    .with_prompt("Select an option.")
    .default(0)
    .items(&choices)
    .interact()
    .unwrap();

    match choices[selection] {
        "New Password" => check_for_file(),
        "Grab Password" => grab_user_info(),
        "Exit" => close_tool(),
        &_ => todo!() // handle all other possible cases
    }
    println!("You chose: {}", choices[selection]);
}

fn add_user_info() {
    // adding the username and password to the file
    println!("adding user info")
}

fn check_for_file() {
    // going to check to see if the file exist or not
    println!("checking for file")
    add_user_info()
}

fn grab_user_info() {
    println!("grabbing user information")
}

fn close_tool() {
    println!("closing file")
}
