#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true // dummy implementation
}

fn close(f: &mut File) -> bool {
    true // dummy implementation
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_len = tmp.len();

    save_to.reserve(read_len);
    save_to.append(&mut tmp);

    read_len
}

fn get_file_data() {
    let mut file_data = File {
        name: String::from("sample.txt"),
        data: vec![114, 117, 115, 116],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut file_data);
    let file_len = read(&file_data, &mut buffer);
    close(&mut file_data);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file_data);
    println!("file_len: {}", file_len);
    println!("text: {}", text);
}

fn main() {
    get_file_data();
}
