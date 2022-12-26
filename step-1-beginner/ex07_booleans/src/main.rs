fn main() {
    let n: i32 = 10;

    println!("{}", n == 10);
    println!("{}", n < 10);
    println!("{}", n > 10);
    println!("{}", n <= 10);
    println!("{}\n", n >= 10);

    let isRaining: bool = false;
    let isLearningRust: bool = true;

    
    println!("{}", isRaining || isLearningRust); // isRaining OR isLearningRust
    println!("{}", isRaining || isRaining);
    println!("{}",  isLearningRust || isRaining);
    println!("{}", isLearningRust || isLearningRust);
    println!("");

    println!("{}", isRaining && isLearningRust); // isRaining AND isLearningRust
    println!("{}", isRaining && isRaining);
    println!("{}",  isLearningRust && isRaining);
    println!("{}", isLearningRust && isLearningRust);

}

/*
(it is raining today) OR (we are learning rust)
F - T = T
F - F = F
T - F = T
T - T = T

(it is raining today) AND (we are learning rust)
F - T = F
F - F = F
T - F = F
T - T = T
 */
