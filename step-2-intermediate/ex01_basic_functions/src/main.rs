fn example_function_2() {
    for i in 1..10 {
        print!("{}", i);
    }
}

fn main() {
    println!("Hello, world!");

    example_function();
    prints_a_greeting();
}

fn example_function() {
    println!("Hello example function");
    example_function_2();
}

fn prints_a_greeting() {
    println!("Hello, good afternoon.");
}
