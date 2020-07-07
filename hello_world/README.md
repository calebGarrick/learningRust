
## [Primitive Types](https://doc.rust-lang.org/stable/rust-by-example/primitives.html)
  
**Scalar Types**  	

- signed int (`i8`, `i16`, `i32`, `ietc`)
- unsgined int (`u8`, `u16`, `uetc`)
- floating pt (`f32`, `f64`)
- `char` 4 byte unicode i.e. `'a'`
- `bool` is `true` or `false`
- The unit type `()` is an empty tuple which is considered a 
scalar	type since it does not contain multiple values.

**Literals**

- all scalar types and strings may be expressed as a literal
- Integers may still be writting in hex, octal, or binary via 
`0x`, `0o`, or `0b` respectively
- Underscores may be emplaced into numeric literals for readability

	
### Compound Types

[**Arrays and Slices**](https://doc.rust-lang.org/stable/rust-by-example/primitives/array.html)

- Arrays are collections of objects of the same type
- are written as `[1, 2, 3]`
- Size must be known at compile-time `[1u8; 2]`
- Slices have a size unknown at compile time
- Slices contain a pointer to data and length of the slice


**[Tuples](https://doc.rust-lang.org/stable/rust-by-example/primitives/tuples.html)** 

- A collection of values of diff types
- Are written as `myTuple = (1, true, x, y, z)`
- Elements may be accessed like `myTuple.0` (`1`) or `myTuple.1` (`true`)
- Tuples may be deconstructed via `let (x, y) = point`, structs must be deconstructed using the above

**Notes**

- Default behavior is to use snake case (my_var_name 
rather than myVarName)
- Variables are immutable by default unless marked mut
- Operators in rust are similar to other C-based languages

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

