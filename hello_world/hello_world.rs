// Caleb Garrick
// This is my first look at Rust and macros
//	I am worried I dived down a deep rabbit hole

// this macro takes no arguments and prints hello world

/*macro_rules! hello_world {
	() => {
		println!("Hello World");
	};
}*/

/*fn main() {
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

}*/

//testing tuples with activities posted here https://doc.rust-lang.org/stable/rust-by-example/primitives/tuples.html
{
	use std::fmt; // Import `fmt`

	// A structure holding two numbers. `Debug` will be derived so the results can
	// be contrasted with `Display`.
	#[derive(Debug)]
	struct MinMax(i64, i64);

	// Implement `Display` for `MinMax`.
	impl fmt::Display for MinMax {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			// Use `self.number` to refer to each positional data point.
			write!(f, "({}, {})", self.0, self.1)
		}
	}

	// Define a structure where the fields are nameable for comparison.
	#[derive(Debug)]
	struct Point2D {
		x: f64,
		y: f64,
	}

	// Similarly, implement `Display` for `Point2D`
	impl fmt::Display for Point2D {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			// Customize so only `x` and `y` are denoted.
			write!(f, "x: {}, y: {}", self.x, self.y)
		}
	}

	fn main() {
		let minmax = MinMax(0, 14);

		println!("Compare structures:");
		println!("Display: {}", minmax);
		println!("Debug: {:?}", minmax);

		let big_range =   MinMax(-300, 300);
		let small_range = MinMax(-3, 3);

		println!("The big range is {big} and the small is {small}",
				small = small_range,
				big = big_range);

		let point = Point2D { x: 3.3, y: 7.2 };

		println!("Compare points:");
		println!("Display: {}", point);
		println!("Debug: {}", point);

		// Error. Both `Debug` and `Display` were implemented, but `{:b}`
		// requires `fmt::Binary` to be implemented. This will not work.
		// println!("What does Point2D look like in binary: {:b}?", point);
	}

}