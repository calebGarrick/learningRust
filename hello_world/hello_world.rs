// Caleb Garrick
// This is my first look at Rust and macros
//	I am worried I dived down a deep rabbit hole

// this macro takes no arguments and prints hello world

/*macro_rules! hello_world {
	() => {
		println!("Hello World");
	};
}*/

fn main() {
  	//hello_world!();

	//testing primitive data
	{
		let mut my_first_b : bool 	= true;
		let my_second_b			= false;

		let first_numeric: i8 = -0b111_1111;

		//default behavior in rust is to use snake case
		//	here I originally used camel case
		//	my_first_b vs myFirstB

		println!("{0}, {1}", my_first_b, my_second_b);

		my_first_b = false; //my_first_b must be mutable else err

		println!("{0}, {1}, {2}", my_first_b, my_second_b, first_numeric);
	}

	//testing tuples
	{
		
	}

}
