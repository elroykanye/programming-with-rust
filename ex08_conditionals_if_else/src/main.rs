fn main() {
    let mut n = -10;

    // -2, 288, -6789, 0
    if n >= 0 {
        n += 1; // n = n + 1;
    } else {
        n += 4; // n = n + 4;
    } // conditional statements

    println!("{}", n);

    let m: i32;

    m = if n >= 0 { 100 } else { -500 }; // conditional expression
    println!("{}", m);

    let k = 11;

    if k >= 0 {
        if k % 2 == 0 { // modulus - mod
            println!("Yahoo");
        } else {
            println!("Wahla");
            if k > 10 {
                println!("Still going");
            }
        }
    }
}
