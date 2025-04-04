use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rng, RngCore};
use std::fs::File;
use std::io::Write;

fn main() {
    let mut data = [0u8; 16];
    rng().fill_bytes(&mut data);
    // println!("{:?}", URL_SAFE_NO_PAD.encode(data));
    let fname = URL_SAFE_NO_PAD.encode(data);
    // Create file for writing
    let mut file = File::create(format!("./{}.bin", &fname)).expect("Unable to create file");

    // Write the data to the file
    file.write(&data).expect("Unable to write to file");
}
