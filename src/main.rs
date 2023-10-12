use serde::{Deserialize, Serialize};
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
    .items(&choices)
    .interact()
    .unwrap();

    println!("You chose: {}", choices[selection]);
}
