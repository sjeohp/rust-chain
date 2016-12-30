extern crate ring;
extern crate untrusted;

use self::ring::{rand, signature};

use std::io::{Read, Write};
use std::fs;
use std::fs::{File};
use std::str;

static WALLET_PATH: &'static str = ".wallet";
static PUBLIC_KEY_PATH: &'static str = ".wallet/id_ed25519.pub";
static PRIVATE_KEY_PATH: &'static str = ".wallet/id_ed25519";

fn gen_keypair(public_key: &mut [u8; 32], private_key: &mut [u8; 32])
{
    let rng = rand::SystemRandom::new();
    let (_, generated_bytes) = signature::Ed25519KeyPair::generate_serializable(&rng).expect("Failed generating key pair");
    *public_key = generated_bytes.public_key;
    *private_key = generated_bytes.private_key;
}

pub fn get_or_gen_wallet(mut public_key: &mut [u8; 32], mut private_key: &mut [u8; 32])
{
    match fs::metadata(WALLET_PATH)
    {
        Ok(_) => {
            let mut public_key_file = File::open(PUBLIC_KEY_PATH).expect("Failed to open public key file");
            let mut private_key_file = File::open(PRIVATE_KEY_PATH).expect("Failed to open private key file");
            public_key_file.read_exact(public_key).expect("Failed reading public key from file");
            private_key_file.read_exact(private_key).expect("Failed reading private key from file");
        }
        Err(_) => {
            fs::create_dir(WALLET_PATH).expect("Failed creating wallet directory");
            let mut public_key_file = File::create(PUBLIC_KEY_PATH).expect("Failed creating public key file");
            let mut private_key_file = File::create(PRIVATE_KEY_PATH).expect("Failed creating private key file");
            gen_keypair(&mut public_key, &mut private_key);
            public_key_file.write_all(public_key).expect("Failed writing public key to file");
            private_key_file.write_all(private_key).expect("Failed writing private key to file");
        }
    }
}

pub fn signature(bytes: &[u8]) -> Vec<u8>
{
    let mut public_key: [u8; 32] = [0; 32];
    let mut private_key: [u8; 32] = [0; 32];
    get_or_gen_wallet(&mut public_key, &mut private_key);

    let mut sig = [0; 64];
    let sigbytes = &signature::Ed25519KeyPair::from_bytes(&private_key, &public_key).unwrap().sign(bytes);
    sig.clone_from_slice(sigbytes.as_slice());
    sig.to_vec()
}

pub fn verify(bytes: &[u8], sig: &[u8], public_key: &[u8]) -> bool
{
    let key = untrusted::Input::from(&public_key);
    let msg = untrusted::Input::from(bytes);
    let sig = untrusted::Input::from(sig);
    match signature::verify(&signature::ED25519, key, msg, sig)
    {
        Ok(_) => { true }
        Err(_) => { false }
    }
}

pub fn init_wallet()
{
    let mut public_key: [u8; 32] = [0; 32];
    let mut private_key: [u8; 32] = [0; 32];

    get_or_gen_wallet(&mut public_key, &mut private_key);
}
