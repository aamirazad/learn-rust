fn main() {
    // Strings!
    // string literals is what we've seen before
    // like in here
    println!("String literal!");
    // The String type is different that it
    // is not required to be immutable
    let s = String::from("hello");
    println!("{s}");
    let mut message = String::from("hello");
    message.push_str(", world!");
    println!("{message}");
    let s1 = String::from("hello");
    // s2 takes the pointer and s1 is now invalid
    let s2 = s1;
    println!("{s2}");
    // This gives the error
    // borrow of moved value: `s1`
    // println!("{s1}, world!");
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    // We do not need to clone variables of known
    // size (Copy trait) like numbers

    // ownership

    let a = String::from("hello");
	// string moves into the function
    takes_ownership(a);
	

	let x = 5;
	makes_copy(x);
	println!("See {x} is still usable");
	let x2 = gives_ownership();
	let x3 = takes_and_gives_back(x2);
	println!("{x3}");

	let b1 = String::from("hello");
	let (s2, len) = calculate_length(b1);
	println!("{s2}, {len}")
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
	println!("{some_integer}");
}

fn gives_ownership() -> String {
	let some_string = String::from("yours");

	some_string
}

fn takes_and_gives_back(a_string: String) -> String {
	a_string
}

fn calculate_length(s: String) -> (String, usize) {
	let length = s.len();
	(s, length)
}