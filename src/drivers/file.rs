use std::{
    io::prelude::Read,
    fs::File,
    path::Path,
    error::Error,
};

pub struct FileDriver {
    pub data: [u8; 4096],
    size: usize,
}

impl FileDriver {
    pub fn from_string(s: &str) -> Result<Self, Box<dyn Error>> {
        let path = Path::new(s);
        FileDriver::from_path(path)
    }

    pub fn from_path(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut data = [0u8; 4096];
        let mut f = File::open(path)?;
        let size = f.read(&mut data)?;
        Ok(FileDriver { data, size })
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
