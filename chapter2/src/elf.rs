use memmap::Mmap;
use std::fmt;
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

impl fmt::Display for ElfIdentification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "---ElfIdentification---
Class       = {}
Data        = {}
Version     = {}
OS/ABI      = {}
ABI Version = {}",
            self.class, self.endianess, self.version, self.os_abi, self.os_abi_version
        )
    }
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

/// File identification in elf header.
#[repr(packed)]
pub struct ElfHeader {
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl fmt::Display for ElfHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(
                f,
                "---ElfHeader---
Type       = {}
Machine    = {}
Version    = {}
Entry      = {}
P Offset   = {}
S Offset   = {}
Flags      = {}
Entry Size = {}
P Size     = {}
P Number   = {}
S Size     = {}
S Number   = {}
Index      = {}",
                self.e_type,
                self.e_machine,
                self.e_version,
                self.e_entry,
                self.e_phoff,
                self.e_shoff,
                self.e_flags,
                self.e_ehsize,
                self.e_phentsize,
                self.e_phnum,
                self.e_shentsize,
                self.e_shnum,
                self.e_shstrndx,
            )
        }
    }
}

impl ElfHeader {
    pub fn new(binary: &[u8; 48]) -> ElfHeader {
        return unsafe { std::mem::transmute::<[u8; 48], ElfHeader>(*binary) };
    }
}

pub const ELF64_ADDR_SIZE: usize = std::mem::size_of::<ElfIdentification>();

#[repr(packed)]
pub struct ElfProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl fmt::Display for ElfProgramHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(
                f,
                "---ElfProgramHeader---
Type      = {:x}
Flags     = {}
Offset    = {:x}
V_Addr    = {:x}
P_Addr    = {:x}
File_Size = {:x}
Mem       = {:x}
Align     = {}",
                self.p_type,
                self.p_flags,
                self.p_offset,
                self.p_vaddr,
                self.p_paddr,
                self.p_filesz,
                self.p_memsz,
                self.p_align,
            )
        }
    }
}

impl ElfProgramHeader {
    pub fn new(binary: &[u8; 56]) -> ElfProgramHeader {
        return unsafe { std::mem::transmute::<[u8; 56], ElfProgramHeader>(*binary) };
    }
}

#[repr(packed)]
pub struct ElfSectionHeader {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl fmt::Display for ElfSectionHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(
                f,
                "---ElfSectionHeader---
Name      = {}
Type      = {}
Flags     = {}
Addr      = {:x}
Offset    = {:x}
Size      = {:x}
Link      = {}
Info      = {}
AddRalign = {}
EntSize   = {:x}",
                self.sh_name,
                self.sh_type,
                self.sh_flags,
                self.sh_addr,
                self.sh_offset,
                self.sh_size,
                self.sh_link,
                self.sh_info,
                self.sh_addralign,
                self.sh_entsize,
            )
        }
    }
}

impl ElfSectionHeader {
    pub fn new(binary: &[u8; 64]) -> ElfSectionHeader {
        return unsafe { std::mem::transmute::<[u8; 64], ElfSectionHeader>(*binary) };
    }
}

pub const ELF64_HEADER_SIZE: usize = std::mem::size_of::<ElfHeader>();
pub const ELF64_PROGRAM_HEADER_SIZE: usize = std::mem::size_of::<ElfProgramHeader>();
pub const ELF64_SECTION_HEADER_SIZE: usize = std::mem::size_of::<ElfSectionHeader>();

pub fn get_elf_header(mapped_file: &Mmap) -> ElfHeader {
    let mut header_binary = [0; ELF64_HEADER_SIZE];
    for (i, b) in mapped_file[ELF64_ADDR_SIZE..ELF64_HEADER_SIZE + ELF64_ADDR_SIZE]
        .iter()
        .enumerate()
    {
        header_binary[i] = *b;
    }
    ElfHeader::new(&header_binary)
}

pub fn get_program_headers(mapped_file: &Mmap, elf_header: &ElfHeader) -> Vec<ElfProgramHeader> {
    let mut headers = Vec::<ElfProgramHeader>::new();
    for i in 0..elf_header.e_phnum as usize {
        let mut section_binary = [0; ELF64_PROGRAM_HEADER_SIZE];
        let offset = elf_header.e_phoff as usize + (i * ELF64_PROGRAM_HEADER_SIZE) as usize;
        for (i, b) in mapped_file[offset..offset + ELF64_PROGRAM_HEADER_SIZE]
            .iter()
            .enumerate()
        {
            section_binary[i] = *b;
        }
        let section = ElfProgramHeader::new(&section_binary);
        headers.push(section);
    }
    return headers;
}

pub fn get_section_headers(mapped_file: &Mmap, elf_header: &ElfHeader) -> Vec<ElfSectionHeader> {
    let mut headers = Vec::<ElfSectionHeader>::new();
    for i in 0..elf_header.e_shnum as usize {
        let mut section_binary = [0; ELF64_SECTION_HEADER_SIZE];
        let offset = elf_header.e_shoff as usize + (i * ELF64_SECTION_HEADER_SIZE) as usize;
        for (i, b) in mapped_file[offset..offset + ELF64_SECTION_HEADER_SIZE]
            .iter()
            .enumerate()
        {
            section_binary[i] = *b;
        }
        let section = ElfSectionHeader::new(&section_binary);
        headers.push(section);
    }
    return headers;
}

pub fn get_section_names(
    mapped_file: &Mmap,
    section_headers: &Vec<ElfSectionHeader>,
    elf_header: &ElfHeader,
) -> Vec<String> {
    let header: &ElfSectionHeader = section_headers.get(elf_header.e_shstrndx as usize).unwrap();
    let mut section_binary = Vec::<u8>::new();
    for (_, b) in mapped_file
        [header.sh_offset as usize..header.sh_offset as usize + header.sh_size as usize]
        .iter()
        .enumerate()
    {
        section_binary.push(*b);
    }

    let mut names = Vec::<String>::new();
    for header in section_headers.iter() {
        // +1 ???
        let mut index = header.sh_name as usize + 1;
        let mut bytes = Vec::<u8>::new();
        let mut t = section_binary[index];
        while t != 0x00 {
            bytes.push(t);
            index += 1;
            t = section_binary[index];
        }
        let converted = String::from_utf8(bytes.to_vec()).unwrap();
        names.push(converted);
    }

    return names;
}
