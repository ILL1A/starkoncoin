use std::error;
use std::fmt;

use rand::Rng;
use secp256k1::{PublicKey,SecretKey,rand::{rngs,SeedableRng}};
use hex::{encode as hex_encode, decode as hex_decode};

#[derive(Debug)]
enum ConvertError {
  FromHexError(hex::FromHexError),
  InitKeyError(secp256k1::Error),
}

impl fmt::Display for ConvertError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
        ConvertError::FromHexError(..) =>
              write!(f, "failed to convert string to hex"),
        ConvertError::InitKeyError(..) =>
              write!(f, "failed to initialize key with hex string"),
      }
  }
}

impl error::Error for ConvertError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
      match *self {
        ConvertError::FromHexError(ref e) => Some(e),
        ConvertError::InitKeyError(ref e) => Some(e),
      }
  }
}

impl From<hex::FromHexError> for ConvertError {
  fn from(err: hex::FromHexError) -> ConvertError {
    ConvertError::FromHexError(err)
  }
}

impl From<secp256k1::Error> for ConvertError {
  fn from(err: secp256k1::Error) -> ConvertError {
    ConvertError::InitKeyError(err)
  }
}

pub struct KeyPair {
  secret_key: SecretKey,
  public_key: PublicKey,
}

/// Generate a new secp256k1 keypair
impl KeyPair {
  pub fn generate_keypair() -> KeyPair {
    let secp = secp256k1::Secp256k1::new();
    let mut rng_seed = rand::thread_rng();
    let seed = rng_seed.gen::<u64>();
    let mut rng = rngs::StdRng::seed_from_u64(seed);
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);
    KeyPair{secret_key, public_key}
  }
  /// Convert a secret key to a hex string
  pub fn secret_key_to_string(&self) -> String {
    hex_encode(self.secret_key.secret_bytes()) // Convert to a hexadecimal string
  }

  /// Convert a public key to a hex string
  pub fn public_key_to_string(&self) -> String {
    hex_encode(self.public_key.serialize()) // Convert the public key into a hex string
  }

  /// Convert a hex string back into a secret key
  fn _string_to_secret_key(&mut self, secret_string: &str) -> Result<SecretKey, ConvertError> {
    let decoded_bytes = hex_decode(secret_string)?;
    SecretKey::from_slice(&decoded_bytes).map_err(|err| err.into())
  }

  /// Convert a hex string back into a public key
  fn _string_to_public_key(public_string: &str) -> Result<PublicKey, ConvertError> {
    let decoded_bytes = hex_decode(public_string)?;
    PublicKey::from_slice(&decoded_bytes).map_err(|err| err.into())
  }
}