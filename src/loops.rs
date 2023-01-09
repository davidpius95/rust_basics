// loops - Used to iterate until a codition is met

pub fn run() {
	let mut count = 0;

	//Infinit Loop
	loop{
		count += 1;
		println!("number: {}", count);

		if count == 20 {
			break;
		}
	}

	// while loop (FizzBuzz)

	while count <= 100 {
		if count % 15 == 0{
			println!("fizzBuzz")
		}else if count % 3 ==0 {
			println!("fizz");
		}else if count % 5 == 0 {
			println!("Buzz");
		}else {
			println!("{}",count)
		}
	}

	// for Range 

	for x in 0..100 {
		if x % 15 == 0{
			println!("fizzBuzz")
		}else if x % 3 == 0 {
			println!("fizz");
		}else if x % 5 == 0 {
			println!("Buzz");
		}else {
			println!("{}",x)
		}
	}

}