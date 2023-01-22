use std::{env, fs::File, io::Read};

const BYTES_PER_LINE: usize = 16;

fn main() {
    let arg1 = env::args().nth(1);
    let filename = arg1.expect("usage: hexdump <FILENAME>");

    let mut file = File::open(filename).expect("Unable to open file");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = file.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }

        println!("");
        pos += BYTES_PER_LINE;
    }
}
