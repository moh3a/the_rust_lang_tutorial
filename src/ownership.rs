#[allow(dead_code)]
pub fn run() {
	let x: i32 = 5;
	let _y: i32 = x; // copy

	let s1: String = String:: from("moh3a");
    // let s2: String = s1; // move (not shallow copy) -> error: cannot borrow after a move
    let _s2: String = s1.clone();
	println!("s1 = {}",s1 );

    // s cannot be borrowed after a move
    // beceause s was passed to a function
    // which is like passing s as a variable
    //    let s: String = String::from("hello");
    //    takes_ownership(s);
    //    println!("{}", s);

    let x1: i32 = 5;
    makes_copy(x1); // no error because its a copy -> intergers boolean and caracters are copied instead of moved
    println!("x1 = {}", x1);

    // gives_ownership returns a string, this moves the ownership to the s3 variable
    let s3: String = gives_ownership();
    println!("s3 = {}", s3);

    // we move the value of s4 into the function
    // then returning a string which moves the value back out of the function into the s5 variable
    let s4: String = String::from("bitches");
    let s5: String = takes_and_gives_back(s4);
    println!("s5 = {}", s5);
    
    // the rules of references:
    // 1. at any given time, you can have either one mutable reference
    // or any number of immutable refenrences
    // 2. references must always be valid


    //
    // to use a variable without taking ownership
    // we use a reference
    //
    let s6: String = String::from("mercedes");
    let len: usize = calculates_length(&s6); // we borrow s6 and we dont own it so we cant mutate it
    println!("length of {} is {}", s6, len);

    // trying to mutate a ref
    //    let s7: String = String::from("hello");
    //    change_string_ref(&s7); // error: cannot mutate a borrowed variable
    let mut s7: String = String::from("hello");
    change_string_ref(&mut s7);
    // mutable reference has one big restriction
    // you could only have one immutable reference to particular piece of data in a particular
    // scope
    let _r1: &mut String = &mut s7;
    //    let _r2: &mut String = &mut s7; // error: cannot borrow as mutable more than once

    

    //
    // slices
    // slices lets us reference a contiguous sequence of elements within a collection
    // instead of referencing the entire collection
    // just like references, slices do not take ownership of the underlying data
    //
    let s8: String = String::from("my name is jeff");
    let word: &str = first_word(&s8);
    println!("first word of '{}' is '{}'", s8, word);
    let s9: &str = "my name is yu";
    let word2: &str = first_word(s9);
    println!("first word of '{}' is '{}'", s9, word2);
}

//  fn takes_ownership(some_string: String) {
//      println!("{}", some_string);
//  }

fn makes_copy(some_int: i32) {
    println!("copy x1 = {}", some_int);
}

fn gives_ownership() -> String {
    let some_str: String = String::from("hi there");
    some_str
}

fn takes_and_gives_back(string: String) -> String {
    string
}

fn calculates_length(string: &String) -> usize {
    let length: usize = string.len();
    length
}

//  fn change_string_ref(s: &String) {
//      s.push_str(", moh3a");
//  }

fn change_string_ref(s: &mut String) {
    s.push_str(", moh3a");
}

//
fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
