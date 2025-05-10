use std::env::{Args, args};
use std::fs::{File, FileType, metadata, read, read_to_string};
use std::io::{Error, ErrorKind};

fn read_file(file_path: &String) {
    let file = read(file_path);
    match &file {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("No such file or directory"),
            ErrorKind::IsADirectory => println!("Is a directory"),
            _ => println!("Unkown Error"),
        },
        Ok(File) => {
            print_file(file.unwrap());
        }
    }
}

fn print_file(content: Vec<u8>) {
    for i in content {
        let tmp = char::from(i);
        print!("{tmp}");
    }
}

fn arg_han(args: Args) -> Vec<String> {
    let mut argv = Vec::new();
    for i in args {
        argv.append(&mut vec![i]);
    }
    return argv;
}

fn main() {
    let argv = arg_han(args());
    for i in 1..argv.len() {
        read_file(&argv[i]);
    }
}
