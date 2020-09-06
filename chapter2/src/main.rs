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
    println!("Class   = {}", identification.class);
    println!("Data    = {}", identification.endianess);
    println!("Version = {}", identification.version);
    println!("OS/ABI = {}", identification.os_abi);
    println!("ABI Version = {}", identification.os_abi_version);
}
