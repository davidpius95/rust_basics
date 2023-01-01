use crate::print;

 pub fn run() {
	// print to console
	println!("Hello world ");

	// basic Formatting 
	println!("number {}",2);

	// positional Arguments
	println!("{0} is from {1} and {0} like to {2}", "brad","Mass","code");

	// Named Arguments
	println!("{name} like to play {activity}",
	name = "john",
	activity = "Baseball");

	//Placeholder traits
	println!("Binary: {:b}, Hex: {:x} Octal: {:o}",10,10,10);

	// Placeholder for debug trait
	println!("{:?}", (12, true, "Hello"));

	// Basic Math
	println!(" 10 + 10 = {}", 10 +10);
}