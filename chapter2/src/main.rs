mod elf;
use memmap::Mmap;

fn get_elf_header(mapped_file: &Mmap) -> elf::ElfHeader {
    let mut header_binary = [0; elf::ELF64_HEADER_SIZE];
    for (i, b) in mapped_file[elf::ELF64_ADDR_SIZE..elf::ELF64_HEADER_SIZE + elf::ELF64_ADDR_SIZE]
        .iter()
        .enumerate()
    {
        header_binary[i] = *b;
    }
    elf::ElfHeader::new(&header_binary)
}

fn get_section_headers(
    mapped_file: &Mmap,
    elf_header: &elf::ElfHeader,
) -> Vec<elf::ElfSectionHeader> {
    let mut headers = Vec::<elf::ElfSectionHeader>::new();
    for i in 0..elf_header.e_shnum as usize {
        let mut section_binary = [0; elf::ELF64_SECTION_HEADER_SIZE];
        let offset = elf_header.e_shoff as usize + (i * elf::ELF64_SECTION_HEADER_SIZE) as usize;
        for (i, b) in mapped_file[offset..offset + elf::ELF64_SECTION_HEADER_SIZE]
            .iter()
            .enumerate()
        {
            section_binary[i] = *b;
        }
        let section = elf::ElfSectionHeader::new(&section_binary);
        headers.push(section);
    }
    return headers;
}

fn main() {
    let loader = match elf::ElfLoader::try_new("c_src/elfsamp") {
        Ok(loader) => loader,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
    if !loader.is_elf() {
        println!("Not ELF file!");
        return;
    }
    let identification = elf::ElfIdentification::new(&loader.mapped_file);
    println!("{}", identification);

    let header = get_elf_header(&loader.mapped_file);
    println!("{}", header);
    let section_headers = get_section_headers(&loader.mapped_file, &header);
    for section_header in section_headers {
        println!("{}", section_header);
    }
}
