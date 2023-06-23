use ring::digest;


pub fn hash_password(password: &[u8]) -> Vec<u8> {
	let digest = digest::digest(&digest::SHA256, password);
	let hashed_password = digest.as_ref().to_vec();
	hashed_password
}


use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;


pub fn encrypt(pw: String, master_password: String) -> Vec<u8> {
	let key = derive_key_from_password(master_password);
	let iv = hex!("000102030405060708090a0b0c0d0e0f");
	let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
	cipher.encrypt_vec(pw.as_bytes())
}

pub fn decrypt(encrypted_pw: Vec<u8>, master_password: String) -> String {
	let key = derive_key_from_password(master_password);
	let iv = hex!("000102030405060708090a0b0c0d0e0f");
	let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
	let decrypted_message = cipher.decrypt_vec(&encrypted_pw).unwrap();
	String::from_utf8(decrypted_message).unwrap()
}

fn derive_key_from_password(master_password: String) -> [u8; 16] {
	let mut key = [0; 16];
	for (i, byte) in master_password.bytes().enumerate() {
		key[i % 16] ^= byte;
	}
	key
}
