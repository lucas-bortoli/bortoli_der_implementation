pub mod bit;
pub mod cipher;
pub mod tables;

fn main() {
    cipher::process(0x11F, 0x11F, cipher::DESMode::Decipher);
}
