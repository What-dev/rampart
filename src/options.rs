use ring::digest;
use crate::lockdown;
fn hash_password(password: &[u8]) -> Vec<u8> {
	let digest = digest::digest(&digest::SHA256, password);
	let hashed_password = digest.as_ref().to_vec();
	hashed_password
}
use std::fs;
use std::io::{stdin}; // for print!()

// for metadata()
use serde::{Serialize, Deserialize}; // for User struct
#[derive(Serialize, Deserialize,)]
pub struct User {
	pub master_password: Vec<u8>,
}

fn printpws(entered_password: String){
	#[derive(Deserialize)]
	struct Password {
		pub name: String,
		pub username: String,
		pub password: Vec<u8>,
	}

	let vault_path = "rampart/vault.json";
	let vault_content = fs::read_to_string(vault_path).unwrap();
	let stored_vault: Result<Vec<Password>, serde_json::Error> = serde_json::from_str(&vault_content);
	match stored_vault {
		Ok(vault) => {
			for password in &vault {
				let index = vault.iter().position(|x| x.name == password.name).unwrap();
				println!(">----------------------------------------<");
				println!(" >> Password {}.", index+1);
				println!(" || Name: {}", password.name);
				println!(" || Username: {}", password.username);
				println!(" || Password: {}", lockdown::decrypt(password.password.clone(), entered_password.clone()));
				println!(">----------------------------------------<");
				println!();
			}
		}
		Err(err) => {
			eprintln!(" Failed to deserialize Password object: {}", err);
		}
	}
}

pub fn listpw() {
	let file_path = "rampart/master.json";
	let master_password = rpassword::prompt_password("Enter master password to decrypt: ").unwrap();
	let hashed_password = hash_password(master_password.as_bytes());

	let file_content = fs::read_to_string(file_path).unwrap();
	let stored_user: User = serde_json::from_str(&file_content).expect("Failed to deserialize User object");

	if hashed_password != stored_user.master_password {
		println!(" Incorrect password! Try again or delete rampart.json to start over (this will delete all saved passwords!).");
		return;
	} else {
		println!("                      --Correct password!--");
	}

	let file_path = "rampart/vault.json";
	let new_user = fs::metadata(file_path).is_err();
	if new_user {
		fs::File::create(file_path).expect(" Failed to create file.");
	}
	let file_content = fs::read_to_string(file_path).unwrap();

	if file_content.is_empty() {
		println!(" You have no passwords saved.");
		return;
	}

	printpws(master_password);

}

pub fn addpw() {
	#[derive(Deserialize, Serialize)]
	struct Password {
		pub name: String,
		pub username: String,
		pub password: Vec<u8>,
	}

	let vault_path = "rampart/vault.json";
	let file_path = "rampart/master.json";
	let new_user = fs::metadata(vault_path).is_err();
	if new_user {
		fs::File::create(vault_path).expect("Failed to create file.");
	}

	let master_password = rpassword::prompt_password("Enter master password: ").unwrap();
	let hashed_password = hash_password(master_password.as_bytes());
	let file_content = fs::read_to_string(file_path).unwrap();
	let stored_user: User = serde_json::from_str(&file_content).expect(" Failed to deserialize User object");

	if hashed_password != stored_user.master_password {
		println!(" Incorrect password! Try again or delete rampart.json to start over (this will delete all saved passwords!).");
		return;
	} else {
		println!("                      --Correct password!--");
	}
	// Read the existing data from the vault.json file
	let file_content = fs::read_to_string(vault_path).unwrap_or_else(|_| "[]".to_string());
	let mut stored_vault: Vec<Password> = serde_json::from_str(&file_content).unwrap_or_default();

	loop {
		println!("Enter the name of the password you want to add.");
		let mut name = String::new();
		stdin().read_line(&mut name).unwrap();
		name = name.trim().to_string();
		println!("Enter the username for {}.", name);
		let mut username = String::new();
		stdin().read_line(&mut username).unwrap();
		username = username.trim().to_string();

		println!("Enter the password for {}.", username);
		let mut password = String::new();
		stdin().read_line(&mut password).unwrap();
		password = password.trim().to_string();
		let encrypted_password = lockdown::encrypt(password, master_password.clone());

		let new_password = Password {
			name,
			username,
			password: encrypted_password,
		};

		// Add the new password to the stored vault
		stored_vault.push(new_password);

		// Serialize the entire vault and write it to the file
		let pw_struct =
			serde_json::to_string_pretty(&stored_vault).expect(" Failed to serialize User object.");
		fs::write(vault_path, pw_struct.as_bytes()).expect(" Failed to write to file.");

		println!("                      --Password added!--");
		println!("Would you like to add another password? (y/n)");
		let mut answer = String::new();
		stdin().read_line(&mut answer).unwrap();
		answer = answer.trim().to_string();
		if answer == "n" {
			break;
		}
	}
}

pub fn delpw(){

	#[derive(Deserialize, Serialize)]
	struct Password {
		pub name: String,
		pub username: String,
		pub password: Vec<u8>,
	}

	let vault_path = "rampart/vault.json";
	let file_path = "rampart/master.json";
	let new_user = fs::metadata(vault_path).is_err();
	if new_user {
		println!(" How the hell do you expect to delete a password? You have no passwords to delete!");
		return;
	}

	let master_password = rpassword::prompt_password("Enter master password: ").unwrap();
	let hashed_password = hash_password(master_password.as_bytes());
	let file_content = fs::read_to_string(file_path).unwrap();
	let stored_user: User = serde_json::from_str(&file_content).expect("Failed to deserialize User object");

	if hashed_password != stored_user.master_password {
		println!(" Incorrect password! Try again or delete rampart.json to start over (this will delete all saved passwords!).");
		return;
	} else {
		println!("                      --Correct password!--");
	}
	printpws(master_password);
	println!("Enter the number of the password you want to delete:");
	let mut answer = String::new();
	stdin().read_line(&mut answer).unwrap();
	answer = answer.trim().to_string();
	let answer: usize = answer.parse().unwrap();
	let file_content = fs::read_to_string(vault_path).unwrap();
	let mut stored_vault: Vec<Password> = serde_json::from_str(&file_content).unwrap_or_default();
	stored_vault.remove(answer - 1);
	let pw_struct =
		serde_json::to_string_pretty(&stored_vault).expect(" Failed to serialize User object.");
	fs::write(vault_path, pw_struct.as_bytes()).expect(" Failed to write to file.");
	println!("                      --Password deleted!--");
}