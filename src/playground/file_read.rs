use std::{fs::{File, Metadata}, io::Read, process::exit};

const FILE_PATH: str = "tests/text.txt";

fn do_thing() {
    let mut file: File = File::open(FILE_PATH).expect("Couldn't open 'text.txt'");
    let file_metadata: Metadata = file.metadata().unwrap();
    let mut buffer = vec![0; file_metadata.len() as usize];

    let bytes_count: usize = file.read(&mut buffer).expect("Couldn't read file");
    
    let buffer_length: usize = buffer.len();
    println!("Buffer Length: {buffer_length}\nBytes Count: {bytes_count}");
    println!("\nFile Bytes:");

    if buffer_length <= 0 {
        println!("Buffer Length is zero!");
        exit(1);
    }

    let mut i: usize = 0;
    while i <= buffer_length - 1 {
        println!("{i}: {}", char::from_u32(buffer[i] as u32).unwrap());
        i += 1;
    }

}