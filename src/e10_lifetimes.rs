// ! &i32           -> a reference
// ! &'a i32        -> a reference with an explicit lifetime
// ! &'a mut i32    -> a mutable refernece with an explicit lifetime

pub fn run() {
    println!("hello from lifetimes");

    let string1 = String::from("abdcd");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("the longest string is {}", result); // * valid
    }

    // let string3 = String::from("abdcd");
    // let result;
    // {
    //     let string4 = String::from("xyz");
    //     result = longest(string3.as_str(), string4.as_str()); // ! borrowed value does not live long enough
    // }
    // println!("the longest string is {}", result); // ! invalid
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// the function above is an arror because we did not specify and generic lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
