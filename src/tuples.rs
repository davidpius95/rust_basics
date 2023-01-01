// Tuples group together value of different types
// max 12 element 

pub fn run(){
	let person: (&str, &str, i8) = ("brad","Mass", 37);
	println!("{} is from {} and is {}", person.0, person.1, person.2);
}