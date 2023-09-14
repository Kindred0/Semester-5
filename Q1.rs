use std::cmp;

fn main() {
    let x = 210;
    let y = 256;
    println!("The two numbers are {} , {}" , x , y);
    let mut deviser = cmp::max(x , y);
    let mut devider = cmp::min(x , y);
    let mut remainder = deviser % devider;
    while remainder != 0 {
        deviser = devider;
        devider = remainder;
        remainder = deviser % devider;
    }
    println!("The gcd of the two numbers are {}" , deviser);
}