<div align="center">
    <img src="images/logo.png" width="150">
</div>

# Heraclitus

[![Crate](https://img.shields.io/crates/v/heraclitus-compiler.svg)](https://crates.io/crates/heraclitus-compiler)
[![API](https://docs.rs/rand/badge.svg)](https://docs.rs/heraclitus-compiler/)


Heraclitus is an open source compiler frontend written in rust. It's going to be used as a basis for programming languages such as [Amber](https://amber-lang.cc) and *Flame*.

> **[Warning]**
> This software is not ready for extended usage.


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