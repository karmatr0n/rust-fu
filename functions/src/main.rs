fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_meassurment(5, 'h');
    statement_and_expression();
    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_meassurment(value: i32, unit_label: char) {
  println!("The meassurment is: {}{}", value, unit_label);
}

fn statement_and_expression() {
    // statement are instructions that performs something 
    //and does not return anything
    let _b = 1; 
    // Expressions are part of statement and returns a value
    let _y = { // the block entire is an expression
        let x = 1;
        x + 1 
    };
    // Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value
    println!("statement and expression");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}