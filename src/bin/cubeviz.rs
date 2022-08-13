use std::fs::File;
use std::io::{self, BufReader, Read};
use structopt::StructOpt;

use cubeviz::parser::parse;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "INPUT", default_value = "-")]
    input: String,
}

fn main() {
    let opt = Opt::from_args();
    let content = cat(&opt.input);
    if let Ok(cube) = parse(&content) {
        let svg = cube.tosvg();
        println!("{}", svg);
    } else {
        panic!("Some Error on parsing")
    }
}

fn cat(file_name: &String) -> String {
    let mut content = String::new();
    if file_name == "-" {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut content).unwrap();
    } else {
        let file = File::open(&file_name).unwrap();
        let mut buf_reader = BufReader::new(file);
        buf_reader.read_to_string(&mut content).unwrap();
    }
    content
}
