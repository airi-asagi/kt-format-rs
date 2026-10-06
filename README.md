Rust's braces escape syntaxes works the same way as Python's, and depending on the string you're trying to represent, it can significantly reduce readability.
```rust
assert_eq!(format!("{{{{{}", 1), r"{{1");
```

This crate aims to improve your code's readability by introducing a Kotlin-like escape to `format!` and several related macros.
```rust
use kt_format::kt;

assert_eq!(kt::format!("{{${}", 1), r"{{1"); 
assert_eq!(kt::format!($$"{{$${}", 1), r"{{1"); 
assert_eq!(kt::format!($$"${${$${}", 1), r"${${1"); 
assert_eq!(format!("${{${{{}", 1), r"${${1");
```

For example, within this crate's source code, regular expressions are generated using code like the following:
```rust
Regex::new(&std::format!(r"\${{{}}}\{{\{{([^{{}}]*?)\}}\}}", n)).unwrap()
```

Although this is actually impossible due to proc_macro constraints, if I could use this crate here, I could write it as follows.
```rust
Regex::new(&std::format!($$r"\${$${}}\{\{([^{}]*?)\}\}", 3)).unwrap() 
```
