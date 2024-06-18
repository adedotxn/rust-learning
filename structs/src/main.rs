struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// Unlike functions, methods are defined within the context of a struct (or an enum or a trait object
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut current_user = User {
        active: true,
        username: String::from("adigunjale"),
        email: String::from("adigunjale@yahoo.com"),
        sign_in_count: 1,
    };

    current_user.sign_in_count = 2;

    let user2 = User {
        username: String::from("adigunjalex2"),
        ..current_user
    };

    // we can no longer use current_user as a whole after creating user2
    // because the String in the email field of current_user was moved into user2.
    //If we had given user2 new String values for both email and username,
    // and thus only used the active and sign_in_count values from current_user,
    // then current_user would still be valid after creating user2.

    /* Tuple Structs */
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Each struct you define is its own type, even though the fields within the struct might have the same types.

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sqr_rect = Rectangle::square(10);

    println!("Sqr react is {:#?}", sqr_rect);
}
