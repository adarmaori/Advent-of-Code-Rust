use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}



fn main() {
    let zero: u16 = '0' as u16;
    let inputs = read_lines("input_file.txt");
    let mut result = 0;
    for input in inputs {
        for i in 0..input.len() {
            let c = input.chars().nth(i).unwrap() as u16;
            if c - zero < 10 && c - zero > 0 {
                result += (c - zero) * 10;
                break;
            }
        }        
        for i in (0..input.len()).rev() {
            let c = input.chars().nth(i).unwrap() as u16;
            if c - zero < 10 && c - zero > 0 {
                result += c - zero;
                break;
            }
        }
    }
    println!("{result}");
}
