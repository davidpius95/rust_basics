// Variable hold primitive data or reference to data 
// Variable are immutable by default 
// Rust is a block-scoped language 

pub fn run() {
	let name = "Brad";
	let mut age = 40;
	println!("my name is {} and I am {}", name, age);
	 age = 20;
	println!("my name is {} and I am {}", name, age);
	println!("My name is {}", name);

	// Define Constant
	const ID: i32 = 001;
	println!("ID:{}", ID);
	
	// Assign multiple vars 
	let (my_name, my_age) = ("Brad", 37);
	println!("my name is {} and age is {}",my_name,my_age);
}