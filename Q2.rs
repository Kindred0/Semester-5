use std::cmp;



fn main() {
    let a = 210;
    let b = 256;
    let x = cmp::max(a , b);
    let y = cmp::min(a , b);
    let mut R = GCDExtended(x ,y);
    println!("gcd({},{} = {})" , x , y , R.0);
    println!("{} * {} + {} * {} = {}" , x , R.1 , y , R.2 , R.0);
}

fn GCDExtended(a : i32 , b : i32) -> (i32 , i32 , i32) {
    if a == 0 {
        return (b , 0 , 1 );
    }
    
    let R = GCDExtended(a % b , b ) ; 
    let gcd = R.0;
    let x1 = R.1;
    let y1 = R.2;
    
    let x = y1 - (b / a) * x1;
    let y = x1;
    
    return (gcd , x , y);
}