use crate::crypto;

pub fn generate_account() {
  let account = Account::new();
  
  println!("Generated Secret Key: {:?}", account.key_pair.secret_key_to_string());
  println!("Generated Public Key: {:?}", account.key_pair.public_key_to_string());
}

pub struct Account {
  key_pair: crypto::KeyPair,
}

impl Account {

  pub fn new() -> Self {
    Self {
      key_pair: crypto::KeyPair::generate_keypair()
    }
  }
}
