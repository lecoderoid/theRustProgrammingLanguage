fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the value of result is {result}");

    println!("\nLOOP LABEl");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);

    println!("\nWhile Loop");

    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("LIFTOFF!");

    println!("\nFor Loop");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    println!("\nUsing `Range` in for loop");
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!");
}
