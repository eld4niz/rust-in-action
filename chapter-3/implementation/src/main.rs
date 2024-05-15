#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &[u8]) -> File {
        let mut file = File::new(name);
        file.data = data.to_vec();
        file
    }

    fn read(&self, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_len = tmp.len();

        save_to.reserve(read_len);
        save_to.append(&mut tmp);

        read_len
    }
}

fn open(f: &mut File) -> bool {
    true // dummy implementation
}

fn close(f: &mut File) -> bool {
    true // dummy implementation
}

fn get_file_data() {
    let mut file_data = File::new_with_data("sample.txt", &[114, 117, 115, 116]);

    let mut buffer: Vec<u8> = vec![];

    open(&mut file_data);
    let file_len = file_data.read(&mut buffer);
    close(&mut file_data);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file_data);
    println!("file_len: {}", file_len);
    println!("text: {}", text);
}

fn main() {
    get_file_data();
}
