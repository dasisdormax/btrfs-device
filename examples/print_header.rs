use std::fs::File;
use std::error::Error;

use btrfs_device::Btrfs;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/example.img")?;
    let mut btrfs = Btrfs::new(file);
    let header = btrfs.get_header()?;
    println!("{:?}", header);
    Ok(())
}