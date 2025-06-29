use std::env::{Args, args};
use std::fs::{File, read};
use std::io::ErrorKind;
use std::{fmt, path::PathBuf};

#[derive(Debug)]
enum myError {
    Silent,
    NotFound(PathBuf),
    IsADirectory(PathBuf),
    CommandNotFound(PathBuf),
}


impl fmt::Display for myError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            myError::Silent => Ok(()),
            myError::NotFound(p) => {
                write!(f, "'{}': No such file or directory", p.display())
            }
            myError::IsADirectory(p) => write!(f, "'{}': Is a directory", p.display()),
            myError::CommandNotFound(p) => write!(f, "'{}': Command Not Found", p.display()),
        }
    }
}

fn read_file(file_path: &PathBuf) -> Result<Vec<u8>, myError> {
    let file = read(file_path);
    match &file {
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(myError::NotFound(file_path.clone())),
            ErrorKind::IsADirectory => Err(myError::IsADirectory(file_path.clone())),
            _ => Err(myError::CommandNotFound(file_path.clone())),
        },
        Ok(File) => Ok(file.unwrap()),
    }
}

fn print_file(content: Vec<u8>) {
    for i in content {
        let tmp = char::from(i);
        print!("{tmp}");
    }
}


fn show_ends(content: &mut Vec<u8>) {
    let mut content_length = content.len();
    let mut index = 0;
    while index < content_length {
        let tmp = char::from(content[index]);
        if tmp == '\n' {
            content.insert(index, 36);
            content_length += 1;
            index += 1;
        }
        index += 1;
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
        let first_char = argv[1].chars().nth(0).unwrap();
        if first_char == '-' {
            println!("is a command");
        } else {
            let mut tmp = read_file(&PathBuf::from(&argv[i]));
            match tmp {
                Err(e) => println!("cat: {}", e),
                Ok(mut o) => {
                    print_file(o);
                }
            }
        }
    }
}
