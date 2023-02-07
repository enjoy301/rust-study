#![allow(unused_variables)]

use std::fmt;
use std::fmt::{Display};

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Close,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Close => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    pub fn new(name: &str, data: &Vec<u8>) -> File {
        File {
            name: String::from(name),
            data: data.clone(),
            state: FileState::Close,
        }
    }

    pub fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("error ㅡㅡ"));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }

    pub fn open(self: &mut File) -> Result<String, String> {
        self.state = FileState::Open;

        Ok(String::from("good"))
    }

    fn close(self: &mut File) -> Result<String, String> {
        self.state = FileState::Close;

        Ok(String::from("good"))
    }
}



fn main() {
    let f2_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f2 = File::new("2.txt", &f2_data);

    let mut buffer: Vec<u8> = vec![];

    f2.open().unwrap();
    let f2_length = f2.read(&mut buffer).unwrap();
    f2.close().unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{}", f2);
    println!("{} is {} bytes", &f2.name, f2_length);
    println!("{}", text);
}
