<div align="center">
    <img src="images/logo.png" width="150">
</div>

# Heraclitus

[![Crate](https://img.shields.io/crates/v/heraclitus-compiler.svg)](https://crates.io/crates/heraclitus-compiler)
[![API](https://docs.rs/rand/badge.svg)](https://docs.rs/heraclitus-compiler/)


Heraclitus is an open source compiler frontend written in rust. It's going to be used as a basis for programming languages such as [Amber](https://amber-lang.cc) and *Flame*.

## Heraclitus - the compiler frontend

With heraclitus you can create your language by skipping the cumbersome lexing step
and using convenience parsing methods that can get you started on your language much quicker.

The main construct that you need is the `Compiler`. The compiler will tokenize your code and assemble it
in a way that you can use to create AST by implementing predefined trait that helps you parse your code.

It's pretty simple. In order to get started you need 3 steps:
1. Create lexing rules
2. Create your ast nodes and let them implement trait provided by this package
3. Create compiler and tie all the components together

Voilá! 🎉

Now you got yourself a ready to analyze / interpret / validate / compile AST.

Ready to get started?

## Example
```rust
use heraclitus::prelude::*;
Compiler::new("HerbScript", rules);
```
It is recommended to use included prelude to import just the things we will actually need.

The `Compiler` requires lexer rules in order to exist.

```rust
let cc = Compiler::new("HerbScript", rules);
let tokens = cc.tokenize()?;
```

# Change log 🚀

## Version 1.3.0
### Fix:
- Added support for UTF symbols

## Version 1.2.6
### Fix:
- Critical bug with non-tokenizable regions being tokenized

## Version 1.2.5
### Feature:
- Changed Logger API that improves adding code snippets
- `Logger::new_err` is now called `Logger::new_err_at_position`
- `Logger::new_warn` is now called `Logger::new_warn_at_position`
- `Logger::new_info` is now called `Logger::new_info_at_position`

## Version 1.2.4
### Fix:
- Bad token highlighting in Logger when showing a code snippet

## Version 1.2.2
### Fix:
- Major bug with a tokenised interpolation being parsed in a wrong way.
- Intensively used code is now inlined at compile time

## Version 0.2.1
### Feature:
- `ErrorDetails::from_token_option(...)` can now be used to create errors at location of given token

## Version 0.2.0
### Feature:
- Added compounds
- Logger can now display messages not related to code
- New method for retrieving current token
- New debug functionality

### Fix:
- Changed string reference of all function parameters to `impl AsRef<str>`
