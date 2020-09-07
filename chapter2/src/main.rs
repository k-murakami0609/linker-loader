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
    let mut binary: [u8; 48] = [0; 48];
    for (i, b) in loader.mapped_file[elf::ELF64_ADDR_SIZE..48 + elf::ELF64_ADDR_SIZE]
        .iter()
        .enumerate()
    {
        binary[i] = *b;
    }
    let header = elf::ElfHeader::new(&binary);
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
}
