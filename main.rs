// Caleb Garrick
// This is my first look at Rust and macros
//	I am worried I dived down a deep rabbit hole


macro_rules! print_num {
	($string:) => {
		println!("Hello World");
	};
}

fn main() {
  hello_world!()
}
