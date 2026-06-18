use getrandom::{SysRng, rand_core::UnwrapErr};
use pallas_crypto::key::ed25519::SecretKey;

pub mod ed25519_bip32;

fn main() {
    println!("Hello, world!");
    let key = SecretKey::new(UnwrapErr(SysRng));
}
