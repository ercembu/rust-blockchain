use ethereum_types::{Address, H160};
use lazy_static::lazy_static;
use rand;
use secp256k1::{All, PublicKey, Secp256k1, SecretKey, generate_keypair};
use sha3::{Digest, Keccak256};

lazy_static! {
    pub(crate) static ref CONTEXT: Secp256k1<All> = Secp256k1::new();
}

pub fn hash(bytes: &[u8]) -> [u8; 32] {
    Keccak256::digest(bytes).into()
}
pub fn to_address(item: &[u8]) -> H160 {
    let hash = hash(&item[1..]);
    Address::from_slice(&hash[12..])
}

pub fn random_keypair() -> (SecretKey, PublicKey) {
    generate_keypair(&mut rand::thread_rng())
}

///Trait for shared Address utility functions
pub trait AddressUtil {
    fn to_address(&self) -> H160;
}

///Convert public key to an address by using the last 20 bytes
impl AddressUtil for PublicKey {
    fn to_address(&self) -> H160 {
        let item = self.serialize_uncompressed();
        let hash = hash(&item[1..]);
        Address::from_slice(&hash[12..])
    }
}

impl AddressUtil for SecretKey {
    fn to_address(&self) -> H160 {
        let public_key = self.public_key(&CONTEXT);
        public_key.to_address()
    }
}

