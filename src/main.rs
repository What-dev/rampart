mod lockdown;
mod options;

// for hash_password()
use std::fs; // for metadata()
use std::io::prelude::*; // for write_all()
use serde::{Serialize, Deserialize}; // for User struct

#[derive(Serialize, Deserialize,)]

pub struct User {
     pub master_password: Vec<u8>,
}

fn main() {
    let file_path = "rampart/master.json";
    let new_user = fs::metadata(file_path).is_err();
    if new_user {
        println!("Welcome to Rampart!");
        println!("Please enter a master password to get started.");
        let master_password = rpassword::prompt_password("Enter master password: ").unwrap();
        println!("You entered: {}", master_password);
        let hashed_password = lockdown::hash_password(master_password.as_bytes());
        let user = User {
            master_password: hashed_password,
        };
        let user_file = serde_json::to_string(&user).expect("Failed to serialize User object.");

        let mut file = fs::File::create(file_path).expect("Failed to create file.");
        file.write_all(user_file.as_bytes()).expect("Failed to write to file.");

        println!("Encrypted master password has been saved to rampart.json.");
    } else {
        println!("Welcome back to Rampart!");
        println!("Please enter your master password.");
        let master_password = rpassword::prompt_password("Enter master password: ").unwrap();
        let hashed_password = lockdown::hash_password(master_password.as_bytes());

        let file_content = fs::read_to_string(file_path).unwrap();
        let stored_user: User = serde_json::from_str(&file_content).expect("Failed to deserialize User object");

        if hashed_password != stored_user.master_password {
            println!("Incorrect password! Try again or delete rampart.json to start over (this will delete all saved passwords!).");
            return;
        } else {
            println!("Correct password!");
        }
    }


    println!("                             -|             |-
         -|                  [-_-_-_-_-_-_-_-]                  |-
         [-_-_-_-_-]          |             |          [-_-_-_-_-]
          | o   o |           [  0   0   0  ]           | o   o |
           |     |    -|       |           |       |-    |     |
           |     |_-___-___-___-|         |-___-___-___-_|     |
           |  o  ]              [    0    ]              [  o  |
           |     ]   o   o   o  [ _______ ]  o   o   o   [     |
           |     ]              [ ||||||| ]              [     |
           |     ]              [ ||||||| ]              [     |
       _-_-|_____]--------------[_|||||||_]--------------[_____|-_-_
      ( (__________------------_____________-------------_________) )");





loop {
    println!("\n                    Welcome to your Rampart Vault!");
    println!("                      What would you like to do?");
    println!("\n                    +++++++++++++++++++++++++++++++++");
    println!("                        1. Add a new password");
    println!("                        2. List all passwords");
    println!("                        3. Delete a password");
    println!("                        (Nothing): Exit");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    match input {
        "1" => {
            options::addpw();
        }
        "2" => {
            options::listpw();
        }
        "3" => {
            options::delpw();
        }
        _ => {break;}
    }

}
}

