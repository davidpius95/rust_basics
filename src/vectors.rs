
// Array -Fixed list where element are same data types
// Vectors are resizable Arrays
use std::mem;
pub fn run() {
	let mut  numbers: Vec<i32> = vec![1,2,3,4,5];


	// add on to Vector
	numbers.push(6);
	numbers.push(8);

	// pop off last value
	numbers.pop();

	println!("{:?}", numbers);

	// Get single val
	println!("{}",numbers[0]);

	// Re-assign value
	numbers[2] = 20;

	//Get vector length
	println!("Array Length: {}", numbers.len());

	// vector are stack allocated 
	println!("Array occupies {} bytes", mem::size_of_val(&numbers));

	// Get slice
	let slice: &[i32] = &numbers[0..2];
	println!("slice {:?}",slice);

	// loop through vector value 
	for x in numbers.iter_mut() {
		println!("Number: {}", x);
		*x *=2;
	}
	println!("Numbers Vec: {:?}", numbers);
}