# learningRust


### Primitive Types
  

### Macros
  According to [rust-lang](https://doc.rust-lang.org/stable/rust-by-example/macros.html),
Macros are expanded into source code this is similar behavior to C.
Rust macros are expanded into abstract syntax trees, rather than string 
preprocessing, so you don't get unexpected precedence bugs.

Macros are useful for 
- DRY coding
- Domain-specific languages
- Variadic interfaces, one that might take a varying # of args i.e. println

Macro arguments are prefixed as $<identInMacro> and 
type suffixed as :<designator>
See [metavariables](https://doc.rust-lang.org/reference/macros-by-example.html)
