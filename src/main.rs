use std::{env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        length if length > 1 => {
            println!("Usage: jlox [script]");
            exit(64);
        }
        1 => run_file(&args[0]),
        _ => run_prompt(),
    }
}

fn run_file(file_path: &String) {
    let bytes = fs::read(file_path);

    let file = match bytes {
        Ok(bytes) => String::from_utf8(bytes),
        Err(_) => panic!("Couldn't read the input file."),
    };

    match file {
        Ok(file) => run(file),
        Err(_) => panic!("Couldn't parse file into utf-8 encoding."),
    }
}

fn run_prompt() {
    unimplemented!()
}

fn run(file: String) {
    unimplemented!()
}
