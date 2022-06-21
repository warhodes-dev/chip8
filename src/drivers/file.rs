use std::{
    io::prelude::Read,
    fs::File,
    path::Path,
    error::Error,
};

const ROM_SIZE: usize = 4096 - 0x200;

pub struct FileDriver {
    pub data: [u8; ROM_SIZE],
}

impl FileDriver {
    pub fn from_string(s: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(s);
        FileDriver::from_path(path)
    }

    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut data = [0u8; ROM_SIZE];
        let mut f = File::open(path)?;
        f.read(&mut data)?;
        Ok(FileDriver { data })
    }
}
