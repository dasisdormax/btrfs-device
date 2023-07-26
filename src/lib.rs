use std::io::{Read, Write, Seek, SeekFrom, Result as IOResult};
use btrfs_diskformat::{
    SuperBlock
};
use core::mem::{size_of, transmute};

pub struct Btrfs<T: BtrfsDevice> {
    device: T,
}

impl<T: BtrfsDevice> Btrfs<T> {
    pub fn new(device: T) -> Self {
        Self {
            device
        }
    }

    pub fn get_header(&mut self) -> IOResult<SuperBlock> {
        self.device.seek(SeekFrom::Start(0x10000))?;
        
        let mut data = [0u8; size_of::<SuperBlock>()];
        self.device.read_exact(&mut data)?;
        Ok(unsafe { transmute(data) })
    }
}

pub trait BtrfsDevice : Read + Write + Seek {}

impl<T> BtrfsDevice for T where T: Read + Write + Seek {}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
