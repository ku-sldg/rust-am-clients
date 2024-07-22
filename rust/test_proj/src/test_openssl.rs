use openssl::sign::{Signer, Verifier};
use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;

// Generate a keypair
let keypair = Rsa::generate(2048).unwrap();
let keypair = PKey::from_rsa(keypair).unwrap();

let data = b"hello, world!";
let data2 = b"hola, mundo!";

// Sign the data
let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
signer.update(data).unwrap();
signer.update(data2).unwrap();
let signature = signer.sign_to_vec().unwrap();

// Verify the data
let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
verifier.update(data).unwrap();
verifier.update(data2).unwrap();
assert!(verifier.verify(&signature).unwrap());