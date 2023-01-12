// Enums are types which have a few definate value

	enum Movement {
		// Varient 
		up,
		Down,
		Left,
		Right
	}

	fn move_avatar(m:Movement){
		// perform action depending on info
		match m {
			Movement::up => println!("Avatar moving up"),
			Movement::Down => println!("Avatar moving Down"),
			Movement::Left => println!("Avatar moving Left"),
			Movement::Right => println!("Avatar moving Right")
		}
	}

	pub fn run () {
		let avatar1 = Movement::Left;
		let avatar2= Movement::up;
		let avatar3= Movement::Down;
		let avatar4= Movement::Right;


		 move_avatar(avatar1);
		 move_avatar(avatar2);
		 move_avatar(avatar3);
		 move_avatar(avatar4);

	}