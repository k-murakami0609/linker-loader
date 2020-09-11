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

    let header = elf::get_elf_header(&loader.mapped_file);
    println!("{}", header);
    let section_headers = elf::get_section_headers(&loader.mapped_file, &header);
    for section_header in section_headers.iter() {
        println!("{}", section_header);
    }
    let names = elf::get_section_names(&loader.mapped_file, &section_headers, &header);
    for name in names.iter() {
        println!("{}", name);
    }
}
