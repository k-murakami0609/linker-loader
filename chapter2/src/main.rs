mod elf;

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
    let mut header_binary = [0; elf::ELF64_HEADER_SIZE];
    for (i, b) in loader.mapped_file
        [elf::ELF64_ADDR_SIZE..elf::ELF64_HEADER_SIZE + elf::ELF64_ADDR_SIZE]
        .iter()
        .enumerate()
    {
        header_binary[i] = *b;
    }
    let header = elf::ElfHeader::new(&header_binary);
    println!("Class   = {}", identification.class);
    println!("Data    = {}", identification.endianess);
    println!("Version = {}", identification.version);
    println!("OS/ABI = {}", identification.os_abi);
    println!("ABI Version = {}", identification.os_abi_version);

    unsafe {
        println!("Type = {}", header.e_type);
        println!("Machine = {}", header.e_machine);
        println!("Version = {}", header.e_version);
        println!("Entry = {}", header.e_entry);
        println!("P Offset = {}", header.e_phoff);
        println!("S Offset = {}", header.e_shoff);
        println!("Flags = {}", header.e_flags);
        println!("Entry Size = {}", header.e_ehsize);
        println!("P Size = {}", header.e_phentsize);
        println!("P Number = {}", header.e_phnum);
        println!("S Size = {}", header.e_shentsize);
        println!("S Number = {}", header.e_shnum);
        println!("Index = {}", header.e_shstrndx);
    }

    for i in 0..header.e_shnum as usize {
        let mut section_binary = [0; elf::ELF64_SECTION_HEADER_SIZE];
        let offset = header.e_shoff as usize + (i * elf::ELF64_SECTION_HEADER_SIZE) as usize;
        for (i, b) in loader.mapped_file[offset..offset + elf::ELF64_SECTION_HEADER_SIZE]
            .iter()
            .enumerate()
        {
            section_binary[i] = *b;
        }
        let section = elf::ElfSectionHeader::new(&section_binary);
        println!("----------------");
        unsafe {
            println!("Name = {}", section.sh_name);
            println!("Type = {}", section.sh_type);
            println!("Flags = {}", section.sh_flags);
            println!("Addr = {:x}", section.sh_addr);
            println!("Offset = {:x}", section.sh_offset);
            println!("Size = {:x}", section.sh_size);
            println!("Link = {}", section.sh_link);
            println!("Info = {}", section.sh_info);
            println!("AddRalign = {}", section.sh_addralign);
            println!("EntSize = {:x}", section.sh_entsize);
        }
    }
}
