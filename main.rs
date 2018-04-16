use std::io::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;

fn main() {
    let file = File::open("./hello").unwrap();
    let fin = BufReader::new(file);
    let file_new = File::create("./world").unwrap();
    let mut fout = BufWriter::new(file_new);

    for line in fin.lines() {
        let new_line = ope_line(&line.unwrap());
        fout.write_all((new_line + "\n").as_bytes());
    }
    fout.flush();
}

fn ope_line(line: &String) -> String {
    line.clone()
}
