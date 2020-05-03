use std::io;
use std::time::Instant;



pub fn main() {
    
    let limit: u32 = read_line_u32();
    let num_of_months: u32 = read_line_u32();
    let mut used_mg: u32 = 0;

    for _ in 0..num_of_months {
        used_mg += read_line_u32();
    }

    println!("{}", limit * (num_of_months + 1) - used_mg);
}

fn read_line_u32() -> u32 {
    let stdin = io::stdin();

    let mut val = String::new();
    stdin.read_line(&mut val).expect("Failed to read line");

    let val: u32 = match val.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Err with reading the value {}", val);
            0
        }
    };
    val
}


pub fn test(){
    let inputs  = super::super::read_lines_fm_file("src/tarifa/examples/tarifa.2.in".to_string());
    let output  = super::super::read_lines_fm_file("src/tarifa/examples/tarifa.2.ans".to_string());

    let now = Instant::now();

    let limit: u32 = inputs[0].parse().unwrap();
    let num_of_months: u32 = inputs[1].parse().unwrap();
    let mut used_mg: u32 = 0;

    for ind in 0..(num_of_months as usize) {
        println!("{}", inputs[ind+2].parse ::<u32> ().unwrap());

        used_mg += inputs[ind+2].parse ::<u32> ().unwrap();
    }

    let res = (limit * (num_of_months + 1)) - used_mg;
    println!("{}", res);
    println!("{}", output[0].parse ::<u32> ().unwrap());

    println!("The time elapsed is: {} ms.", now.elapsed().as_millis());
    assert!( res == output[0].parse ::<u32> ().unwrap());

}