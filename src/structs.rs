struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn run() {
    let mut user1 = User {
        email: String::from("moh3a@fuck.com"),
        username: String::from("moh3a"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("moh");

    let user2 = build_user(String::from("zara@mail.com"), String::from("zara"));
    let user3 = User {
        username: String::from("bib"),
        email: String::from("bib@mail.com"),
        ..user2 // the rest of the fields will be completed with whatever values in user2 ..spread operator like typescript
    };
	println!("Hello structs");

    //    let width1 = 30;
    //    let height1 = 50;
    //    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    // to improve readability of code and make it obvious that the w and h are related
    // we can use a struct
    let rect = Rectangle {
        width: 30,
        height: 50
    };
    println!("rect: {:#?}", rect);
    println!("The area of the rectangle is {} square pixels", rect.area());
    let rect1 = Rectangle {
        width: 15,
        height: 55
    };
    let rect2 = Rectangle {
        width: 20,
        height: 40
    };
    println!("rect can hold rect1? {}", rect.can_hold(&rect1));
    println!("rect can hold rect2? {}", rect.can_hold(&rect2));
}

// fn area (width: u32, height: u32) -> u32 {
//    width * height
// }

fn build_user(email:String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
