//This is the main function from where the execution of rust code starts.
fn main() {
    println!("This is from a main function!");
    //call the external function
    another_function();

    //with parameter
    with_parameter(5);

    //checks if the number is even
    let num: i32 = 4;
    let result = is_even(num);

    if result {
        println!("The number {} is even!", num);
    } else {
        println!("The number {} is odd!", num);
    }
}

//This is another function which prints some text.
fn another_function() {
    println!("This is from another function");
}

//This takes a parameter and prints it.
fn with_parameter(x: i32) {
    println!("Here is value of x: {}", x);
}

//This will return boolean value
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        true
    } else {
        false
    }
}
