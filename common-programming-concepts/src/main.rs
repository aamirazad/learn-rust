fn main() {
    let mut x = 6;
    println!("The value of x is: {x}");
    x = 7;
    println!("The value of x is: {x}");
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
	println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
	{
		let x = 21;
		println!("The inner value of x is: {x}");
	}
	println!("The outer value of x is still: {x}");
	let first_string = "Hello world";
	println!("The string: {first_string} has {} characters", first_string.len());
	let string_as_number: u32 = "42".parse().expect("Not a number");
	println!("The string as number plus one is {}", string_as_number + 1);
	let float = 2.0; // f64
	println!("The float divided by 4.0 equals {}", float/4.0);
	let tup = (500, 64, 1);
	let a = [tup.1,3,5,7];
	println!("{}", a[2]);
	for val in a {
		print!("{val} ");
	}
	println!();
	println!("5 + 2 = {}", add_number(5,2));
	// This is a comment
	if x < 5 {
		println!("{x} is less than 5");
	} else {
		println!("{x} is greater than 5");
	}
	// Inline if statement
	let condition = true;
	let result = if condition { 5 } else { 6 };
	println!("{result}");
	// Loops
	let mut count = 0;
	let result = loop {
	    count += 1;
	    if count == 10 {
	        break count;
	    }
	};
	println!("{result}");
	
	while count < 20 {
	    count += 1;
	}
	println!("{count}");
	for number in 1..4 {
	    println!("{number}");
	}
}

fn add_number(x1: i32, x2: i32) -> i32 {
	x1 + x2
}
