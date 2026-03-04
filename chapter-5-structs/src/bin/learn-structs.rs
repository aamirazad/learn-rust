#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // same as username: username
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");

    let user2 = build_user(user1.username, String::from("user2@example.com"));

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };
    println!("{:#?}", user3);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );

    dbg!(rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
