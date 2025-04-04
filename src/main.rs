use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use std::fs::File;
use std::io::Write;

fn main() {
    // Get stdin parameters
    let mut args = std::env::args();
    println!("{:?}", args);
    // Get first argument (program name)
    args.next().expect("No arguments");
    let output_fname = args.next().expect("No arguments");
    let input_string = args.next().expect("No arguments");

    // Open file for writing
    let mut file = File::create(&output_fname).expect("Unable to create file");
    // println!("{:?}", URL_SAFE_NO_PAD.decode(b"_4TUgAAz5jqmpfS0a8to2g"));
    // file.write(&URL_SAFE_NO_PAD.decode(&input_string).unwrap())
    file.write(&URL_SAFE_NO_PAD.decode(&input_string.as_bytes()).unwrap())
        .expect("Unable to write");
}
