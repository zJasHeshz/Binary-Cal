#[macro_use]
extern crate text_io;

fn main() {
    print!("type the floating value : ");
    let num: f32 = read!();
    // seperate the exponent and fraction
    let exponent: i32 = num.round() as i32;
    let fraction: f32 = num.fract();

    let mut binary_frac_vec: Vec<i32> = vec![];
    let mut oct_frac_vec: Vec<i32> = vec![];
    let mut hex_frac_vec: Vec<i32> = Vec::new();

    cal(fraction, &mut binary_frac_vec, 2.0);
    cal(fraction, &mut oct_frac_vec, 8.0);
    cal(fraction, &mut hex_frac_vec, 16.0);
    println!("In Binary : {:b}.{}", exponent, format(binary_frac_vec));
    println!("In OCT : {:o}.{}", exponent, format(oct_frac_vec));
    println!("In HEX : {:X}.{}", exponent, format(hex_frac_vec));
}

fn cal(mut frac: f32, frac_vec: &mut Vec<i32>, oprator: f32) {
    while frac != 0.0 {
        frac = frac * oprator;
        let x: i32 = frac.floor() as i32;
        frac_vec.push(x);
        frac = frac.fract();
    }
}

fn hex_converter(hex_value: i32) -> String {
    match hex_value {
        10 => return "A".to_string(),
        11 => return "B".to_string(),
        12 => return "C".to_string(),
        13 => return "D".to_string(),
        14 => return "E".to_string(),
        15 => return "F".to_string(),
        _ => return "That isn't a hex".to_string(),
    }
}

fn format(frac_vec: Vec<i32>) -> String {
    return frac_vec
        .iter()
        .map(|n: &i32| {
            if *n >= 10 {
                hex_converter(*n)
            } else {
                n.to_string()
            }
        })
        .collect::<String>();
}
