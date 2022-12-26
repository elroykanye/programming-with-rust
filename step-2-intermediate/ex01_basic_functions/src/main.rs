fn print_a_greeting() {
    println!("Hello, good afternoon.");

    let a = 3;
    print!("{}", a);
}

fn main() {
    println!("Hello, world!");

    example_function_1();
    print_a_greeting();
    print_a_name();

    fn print_a_name() {
        println!("Elroy Kanye");
    }
}

fn example_function_1() {
    println!("Hello example function");
    example_function_2();
}

fn example_function_2() {
    for i in 1..11 {
        print!("{} ", i);
    }
}
