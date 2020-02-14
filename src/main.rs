mod tarifa;
use std::io;

use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::io::{BufReader , BufRead , Lines , Error};

fn main() {
    // tarifa::main_file::main();
    // tarifa::main_file::test();
}


 
fn read_lines_fm_file(path: String) -> Vec<String> {
    // the path can be "src/tarifa/examples/tarifa.2.ans"
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    let mut lines: Vec<String> = vec![];

    for line in f.lines() {
        lines.push(line.unwrap());
    }
    lines
}

// fn read_lines<P>(filename: P) -> Result< Lines<BufReader<File>> , Error>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;    //OBS
//     Ok(BufReader::new(file).lines())
// }
