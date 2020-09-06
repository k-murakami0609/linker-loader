use memmap::Mmap;
use std::fs::File;

// 0x7f 'E' 'L' 'F'
const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

pub struct ElfLoader {
    pub mapped_file: Mmap,
}

impl ElfLoader {
    pub fn try_new(file_path: &str) -> std::io::Result<ElfLoader> {
        let file = File::open(&file_path)?;
        Ok(ElfLoader {
            mapped_file: unsafe { Mmap::map(&file)? },
        })
    }

    pub fn is_elf(&self) -> bool {
        self.mapped_file[0..4] == HEADER_MAGIC
    }
}

/// File identification in elf header.
pub struct ElfIdentification {
    pub magic: [u8; 4],
    pub class: u8,
    pub endianess: u8,
    pub version: u8,
    pub os_abi: u8,
    pub os_abi_version: u8,
    pub reserved: [u8; 7], // zero filled.
}

impl ElfIdentification {
    // assumption: `binary` has enough length to read elf identification.
    pub fn new(binary: &[u8]) -> ElfIdentification {
        let mut magic: [u8; 4] = [0; 4];
        for (i, b) in binary[0..4].iter().enumerate() {
            magic[i] = *b;
        }
        ElfIdentification {
            magic,
            class: binary[4],
            endianess: binary[5],
            version: binary[6],
            os_abi: binary[7],
            os_abi_version: binary[8],
            reserved: [0; 7],
        }
    }
}
