use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::fs::File;
use std::io::Write;

fn main() {
    // Open file for writing
    let mut file = File::create("sess.b64url_decoded.bin").expect("Unable to create file");
    // println!("{:?}", URL_SAFE_NO_PAD.decode(b"_4TUgAAz5jqmpfS0a8to2g"));
    file.write(&URL_SAFE_NO_PAD.decode(b"_4TUgAAz5jqmpfS0a8to2g").unwrap())
        .expect("Unable to write");
}
