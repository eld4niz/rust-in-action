use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

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

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_len = tmp.len();
        save_to.reserve(read_len);
        save_to.append(&mut tmp);
        Ok(read_len)
    }

    fn open(file: File) -> Result<File, String> {
        if one_in(10_000) {
            let error_message = String::from("Permission denied");
            return Err(error_message);
        }
        Ok(file)
    }

    fn close(file: File) -> Result<File, String> {
        if one_in(100_000) {
            let error_message = String::from("Interrupted by signal!");
            return Err(error_message);
        }
        Ok(file)
    }
}

fn main() {
    let data = vec![114, 117, 115, 116, 33];
    let file = File::new_with_data("file.txt", &data);

    let mut buffer: Vec<u8> = vec![];

    file.read(&mut buffer).unwrap();

    let file = File::open(file).unwrap();
    let file = File::close(file).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{} is {} bytes long", file.name, buffer.len());
    println!("{}", text);
}
