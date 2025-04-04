use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rng, RngCore};

fn main() {
    let mut data = [0u8; 16];
    rng().fill_bytes(&mut data);
    println!("{:?}", URL_SAFE_NO_PAD.encode(data));
}
