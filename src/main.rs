// #[macro_use]
// extern crate pgp;

use std::fs::File;
// use std::io::{Cursor, Read};

// use pgp::composed::{Deserializable, Message, SignedPublicKey, SignedSecretKey};
use pgp::composed::{SignedPublicKey, Deserializable};
// use pgp::types::KeyTrait;

fn main() {
    println!("Hello, world!");
    let res = SignedPublicKey::from_armor_single(
        File::open("./bigdevbox_public.asc").unwrap(),
    );
    println!("Result: {:?}", res);
}
