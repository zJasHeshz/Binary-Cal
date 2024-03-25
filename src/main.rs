#[macro_use]
extern crate text_io;

fn main() {
    print!("type the floating value : ");
    let i: f32 = read!();
    let num: f32 = i;
    let exponent: i32 = unsafe { num.to_int_unchecked() };
    let fraction: f32 = num.fract();
    let mut binary_frac_vec: Vec<i32> = vec![];
    let mut oct_frac_vec: Vec<i32> = vec![];

    cal(fraction, &mut binary_frac_vec, 2.0);
    cal(fraction, &mut oct_frac_vec, 8.0);
    println!("In Binary : {:b}.{}", exponent, format(binary_frac_vec));
    println!("In OCT : {:o}.{}", exponent, format(oct_frac_vec));
}

fn cal(mut frac: f32, frac_vec: &mut Vec<i32>, oprator: f32) {
    while frac != 0.0 {
        frac = frac * oprator;

        let x: i32 = unsafe { frac.to_int_unchecked() };

        frac_vec.push(x);

        frac = frac.fract();
    }
}

fn format(frac_vec: Vec<i32>) -> String {
    return frac_vec.iter().map(|n| n.to_string()).collect::<String>();
}
