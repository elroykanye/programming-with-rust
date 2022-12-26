fn main() {
    f(2. /* argument */);
    f(-3.7);
    let res = sum(1, 1);
    println!("{}", res);
    println!("{}", two());
}


fn f(x: f32 /*parameter */) {
    let y = 2. * x + 1.;
    println!("{}", y);
}

fn two() -> i32 {
    2
}

fn sum(x: i32, y: i32) -> i32 {
    let result = x + y;
    return result
}

// f(x) = 2x + 1
// f(2) = 2(2) + 1 = 5

// f(x,y) = x + y
