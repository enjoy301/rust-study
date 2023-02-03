#![allow(unused_variables)]

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Close,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    fn new(name: &str, data: &Vec<u8>) -> File {
        File {
            name: String::from(name),
            data: data.clone(),
            state: FileState::Close,
        }
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("error ㅡㅡ"));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;

    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Close;

    Ok(f)
}

fn main() {
    let f2_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f2 = File::new("2.txt", &f2_data);

    let mut buffer: Vec<u8> = vec![];

    f2 = open( f2).unwrap();
    let f2_length = f2.read(&mut buffer).unwrap();
    f2 = close(f2).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes", &f2.name, f2_length);
    println!("{}", text);
}
