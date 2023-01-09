pub fn run() {
	let age = 1;
	let check_id: bool = true;
	let knows_person_of_age = true;

	// if/Else
	if age >= 21 {
		println!("BARTENDER: what would you like to drink");

	}else if age < 21 && check_id || knows_person_of_age{
		println!("Bartender: Sorry, you have to leave");

	}else {
		println!("Bartender: i will need to see your ID")
	}

	// there are no taniry operation in rust 
	// shorthand if
	let is_of_age = if age >= 21 {true} else {false};
	println!("your age {} ", is_of_age);
}