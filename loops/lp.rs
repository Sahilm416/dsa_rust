//Loops in rust

fn main() {
    // loop keyword
    let mut num_1 = 3;
    loop {
        if num_1 == 0 {
            break;
        }
        println!("{}", num_1);
        num_1 -= 1;
    }

    // while loop
    let mut num_2 = 3;
    while num_2 > 0 {
        println!("{}", num_2);
        num_2 -= 1;
    }

    //for loop
    let numbers : [i32;5] = [1, 2, 3, 4, 5];
    for num in numbers {
        println!("{}", num);
    }
}
