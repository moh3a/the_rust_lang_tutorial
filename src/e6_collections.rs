use std::collections::HashMap;

pub fn run() {
    println!("Hello collections");

    /*

    * part 1
    * vectors

    */
    let a = [1, 2, 3];
    println!("a[0] {}", a[0]);
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v[0] {}", v[1]);

    // for creating vectors and directly assigning values to it
    let v2 = vec![1, 2, 3];
    println!("v2[0] {}", v2[2]);

    // vectors just like any other type stored in the heap, we'll be dropped when they go out of scope
    {
        let _v3 = vec![1, 2, 3];
    }
    // _v3 will be dropped here

    let mut v4 = vec![1, 2, 3, 4, 5];

    // let third = &v4[2];
    // v4.push(6);
    // ! cannot borrow `v4` as mutable because it is also borrowed as immutable
    // println!("the third element of v4 is {}", third);

    // access the third element of v4 using the get method
    match v4.get(2) {
        Some(third) => println!("the third element of v4 is {}", third),
        None => println!("there is no third element"),
    }

    // modify everything in a vector - as a mutable reference
    for i in &mut v4 {
        *i += 50;
    }

    // print out everything in a vector - as an immutable reference
    for i in &v4 {
        println!("&mut v4 + 50 = {}", i);
    }

    // vectors should hold value with same type
    // we can work around with enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Text(String::from("Friendly")),
        SpreadsheetCell::Float(3.14),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("it is an integer with value {}", i),
        SpreadsheetCell::Float(f) => println!("it is a float with value {}", f),
        SpreadsheetCell::Text(s) => println!("it is a text with value {}", s),
    }

    /*

    * part 2
    * strings

    */
    let mut s = String::from("foo"); // foo
    s.push_str("bar"); // foobar // push string slices (&stc)
    s.push('!'); // foobar! // push chars

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // ! let s3 = s1 + &s2;
    // println!("{}", s1); // borrow of moved value s1
    let _s4 = format!("{}{}", s1, s2); // format! doesnt take ownership of s1 and s2 so we can use them without ref

    /*

    * part 3
    * hashmaps

    */
    let blue = String::from("blue");
    let yellow = String::from("yellow");
    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }
}
