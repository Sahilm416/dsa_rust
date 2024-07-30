// Compound data types in rust

fn main() {
    /*
     Rust has several compound data types, including arrays, vectors, strings, tuples, and structs.
    */

    // 1. Tuples
    let person: (&str, u32);
    person = ("Dustin", 36);
    println!("{} is {} years old", person.0, person.1);

    // 2. Arrays
    let numbers: [i8; 5] = [1, 2, 3, 4, 5];
    println!("{}", numbers.len());

    println!("{}", numbers[0]);

    let slice = &numbers[1..];
    println!("{:?}", slice); // {:?} to print all members.

    // 3. Strings
    let name: &str = "Dustin";
    let mut another_name = String::from("Dustin Poirier");
    another_name.push_str(" DP");
    println!("{}", name);
    println!("{}", another_name);
    //concat two strings
    let mut some_text = "some".to_owned() + "thing";
    some_text = some_text.to_owned() + "ok";
    println!("{}", some_text);

    // 4. Vectors
    let mut vec = Vec::new();
    vec.push(3);
    vec.pop();
    println!("{:?}", vec); //empty
}
