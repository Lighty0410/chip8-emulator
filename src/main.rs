mod chip8;

struct ForArray {
    ar: [u8; 4],
}
fn main() {
    let random_array: [u8; 4] = [0; 4];
    println!("{:?}", random_array);

    let data = vec![0; 0x200];

    println!("{:?}", data)
}
