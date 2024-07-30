## Compound data types in rust
1. Tuples
   * tuples are the way of collecting different or same data type elements together.
   * example -> `let tup = (2, "ok");`

2. Array
   * limited in size
   * similar data types needed
   * contiguous memory
   * example -> `let numbers : [i32;3] = [1,2,3];`

3. Strings
   * combination of characters
   * 2 ways of creating string data types
       - `&str`
       - `String::from("abc");`

4. Vectors
   * dynamic behaviour
   * similar data types needed
   * create with
      - `Vec::new();`
      - `vec![1,2,3];`
