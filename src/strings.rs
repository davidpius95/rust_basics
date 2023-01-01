// Primitive sr = Immutable fixed-length string somewhere in memory 
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
	let hello = "Hello";
	println!("{}",hello);

	let mut hell = String::from("Hello");

	// Get length
	println!("Length: {}", hello.len());
	// push char
	hell.push('W');

	// push string
	hell.push_str("world");

	println!("{}",hell);

	// capacity in bytes
	println!("Capacity: {}",hell.capacity());

	// check if empty 
	println!("is Empty: {}", hell.contains("world"));

	// replace 
	println!("replace: {}", hell.replace("world", "there"));

	// loop through string by whitespace

	for word in hell.split_whitespace(){
		println!("{}", word);
	}

	//Create string with capacity 

	let mut s = String::with_capacity(10);
	s.push('a');
	s.push('b');

	// Assertions testing
	assert_eq!(3, s.len());
	println!("{}",s);
	
}