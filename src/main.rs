use std::fs::File;
use pgp::composed::{SignedPublicKey, Deserializable, StandaloneSignature};

fn main() {
    println!("Hello, world!");
    let (public_key, _) = SignedPublicKey::from_armor_single(
        File::open("./bigdevbox_public.asc").unwrap(),
    ).unwrap();

    let (sig, _) = StandaloneSignature::from_armor_single(
        File::open("./cake.txt.sig").unwrap(),
    ).unwrap();

    let res = sig.verify(
        &public_key,
        &std::fs::read("./cake.txt").unwrap(),
    );
    println!("Verify cake.txt cake.txt.sig: {:?}", res);

    let res = sig.verify(
        &public_key,
        &std::fs::read("./pie.txt").unwrap(),
    );
    println!("Verify pie.txt cake.txt.sig: {:?}", res);
}
