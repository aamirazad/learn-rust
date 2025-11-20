fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");
    // Mutable references cannot give out another
    // reference. For example this will fail
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1}");
    // But this works because r1 is not used
    let _r1 = &mut s;
    let r2 = &mut s;
    println!("{r2}");
    // We have to use curly braces to create a new
    // scope in order to make another reference
    let _a = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    // We borrow s here. But since we do not own
    // it, we cannot modify it. For example
    // s.push_str(", world"); would not work
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
    // We cannot return a reference here because
    // the s it is refering to would go out of scope
    // before it is used creating a dangling pointer
}
