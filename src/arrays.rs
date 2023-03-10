
// Array -Fixed list where element are same data types
use std::mem;
pub fn run() {
	let mut  numbers: [i32; 5] = [1,2,3,4,5];

	println!("{:?}", numbers);

	// Get single val
	println!("{}",numbers[0]);

	// Re-assign value
	numbers[2] = 20;

	//Get array length
	println!("Array Length: {}", numbers.len());

	// Array are stack allocated 
	println!("Array occupies {} bytes", mem::size_of_val(&numbers));

	// Get slice
	let slice: &[i32] = &numbers[0..2];
	println!("slice {:?}",slice);
}