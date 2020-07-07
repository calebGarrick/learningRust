# learningRust
___

The purpose of this repo is for me to organize my notes and examples
as I familiarize myself with the Rust language.

One long-term goal is to follow [this](https://rust-unofficial.github.io/too-many-lists/) tutorial.

### [Primitive Types](https://doc.rust-lang.org/stable/rust-by-example/primitives.html)
  
**Scalar Types**  	

	- signed int (`i8`, `i16`, `i32`, `ietc`)
  	- unsgined int (`u8`, `u16`, `uetc`)
  	- floating pt (`f32`, `f64`)
  	- `char` 4 byte unicode i.e. `'a'`
  	- `bool` is `true` or `false`
  	- The unit type `()` is an empty tuple which is considered a scalar	type since it does not contain multiple values.
	
**Compound Types**

	-arrays are written as [1, 2, 3]
	-tuples are writte as (1, true, x, y, z)

### Macros
  According to [rust-lang](https://doc.rust-lang.org/stable/rust-by-example/macros.html),
Macros are expanded into source code this is similar behavior to C.
Rust macros are expanded into abstract syntax trees, rather than string 
preprocessing, so you don't get unexpected precedence bugs.

Macros are useful for 
- DRY coding
- Domain-specific languages
- Variadic interfaces, one that might take a varying # of args i.e. 
println

Macro arguments are prefixed as $<identInMacro> and 
type suffixed as :<designator>
See [metavariables](https://doc.rust-lang.org/reference/macros-by-example.html)

