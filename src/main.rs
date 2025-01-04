mod ext2;
mod layout;

use std::env;
use std::io::Read;

fn main() {
    let filename = env::args()
        .nth(1)
        .or(Some("ext2_image.img".into()))
        .unwrap();
    let mut ext2 = ext2::Ext2::new(&filename);
    let superblock = ext2.parse();
}
