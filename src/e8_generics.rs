#[allow(dead_code)]
pub fn run() {
    println!("hello from generics");

    let number_list = vec![34, 50, 62, 24, 89, 40];
    let largest = get_largest(number_list);

    println!("the largest number of number_list is {}", largest);

    let char_list = vec!['s', 'q', 'a', 'z', 'd'];
    let largest = get_largest(char_list);

    println!("the largest number of char_list is {}", largest);
}

// function takes a generic type
// to be able to compare the values inside the vector we need to restrict our type using traits
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
