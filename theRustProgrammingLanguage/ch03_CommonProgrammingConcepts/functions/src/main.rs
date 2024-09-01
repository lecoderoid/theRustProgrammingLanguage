fn main() {
    println!("Hello, world");
    another_function();
    function_with_param(42);
}

fn another_function() {
    println!("Another function");
}

fn function_with_param(x: i32) {
    println!("Calling function with param");
    println!("The parameter x is {}", x);
}
