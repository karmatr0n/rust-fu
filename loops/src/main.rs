fn main() {
    // loop {
    //     println!("again!");
    // }

    let mut count = 0;

    // Labeled loop
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("Remaining = {}", remaining);
            if remaining == 9 {
                break; // break and continue just work for the inner loop
            }
            if count == 2 {
                break 'counting_up; // Apply break for specific labeled loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // Conditional loops with while
    // This construct eliminates a lot of nesting that would be necessary if you used loop, if, else, and break, and it’s clearer
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    //Looping a collection with a for
    for element in a {
        println!("The value is: {}", element);
    }

    // using a range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
