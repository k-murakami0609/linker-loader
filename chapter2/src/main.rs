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
    println!("{}", identification);

    let header = loader.get_elf_header();
    println!("{}", header);

    let program_headers = loader.get_program_headers();
    for program_header in program_headers.iter() {
        println!("{}", program_header);
    }

    let section_headers = loader.get_section_headers();
    for section_header in section_headers.iter() {
        println!("{}", section_header);
    }
    let names = loader.get_section_names();
    for name in names.iter() {
        println!("{}", name);
    }
}
