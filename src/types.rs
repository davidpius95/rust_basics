/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory )
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

//Rust is a statically typed language, which means that it must know the types of all
// variable at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it. 


pub fn run() {
	// by Default is "i32"
	let x = 1;

	// for floating integer by Default is "f64"
	let y = 2.5;

	// for an explicit type (which means for a type you define)
	let _y: i64 = 454545454545454;

	// find max size 
	println!("max i32: {}", std::i32::MAX);
	println!("max i64: {}", std::i64::MAX);


	// Boolean 
	let is_active = true;

	// Get boolean from expression 
	let is_greater: bool = 10 < 5;
	println!("{:?}", (x,y,_y, is_active, is_greater));

	// character char
	let _a1 = 'a';
	let _face = '\u{1F600}';
	println!("{}", _face);
	
}