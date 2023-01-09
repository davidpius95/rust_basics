// function  - Used to store blocks of code for re-use


pub fn run() {
greeting("hello", "jane");

	// Bind function value to variable
	let get_sum = add(5,5);
	println!("sum: {}", get_sum);


		// Closure
		let add_nums = |mut t1: i32, mut s2: i32| t1 = s2;
		println!("C sum: {:?}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
	println!("{} {}, nice to meet you ", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
	n1 + n2
}

