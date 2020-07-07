// Caleb Garrick
// This is my first look at Rust and macros
//	I am worried I dived down a deep rabbit hole

// this macro takes no arguments and prints hello world

macro_rules! hello_world {
	() => {
		println!("Hello World");
	};
}

fn main() {
  hello_world!()


}
