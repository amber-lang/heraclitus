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

## Version 1.7.5
### Fix:
- Prevent Logger from panicking when trying to display a region that is out of bounds

## Version 1.7.4
### Fix:
- `Logger::text` now doesn't end with a new line
### Feature:
- Added `Logger::line` method that adds a new line in the end (works just like old `Logger::text`)

## Version 1.7.2
### Fix:
- `PositionInfo::from_between_tokens` shows the range even if the end token is None

## Version 1.7.1
### Fix:
- Bugfixes for calculating start index in tokens

## Version 1.7.0
### Feature:
- Tokens now contain information about their index of the first character in the source code
- Added `PositionInfo::from_between_tokens` method to select a region between two tokens in messages

## Version 1.6.2
### Fix:
- Fixes escapes that were handled improperly

## Version 1.6.1
### Fix:
- Remove debug information

## Version 1.6.0
### Fix:
- Heraclitus no longer panics when error happens out of bounds of file

## Version 1.5.9
### Fix:
- Escaping escape key now treats it as a character

## Version 1.5.8
### Fix:
- Escaping regions is now properly handled

## Version 1.5.7
### Fix:
- Major fix that caused the lexer to lead to an undefined behavior with defining a region that has a beginning rule longer than one character.

## Version 1.5.6
### Fix:
- Compiler now does not rely strongly on provided source code. It can now open files from path if provided. This can improve drastically performance of the compiler when working with imports.

## Version 1.5.5
### Feature:
- Show elapsed time in parser debug mode

## Version 1.5.4
### Fix:
- Offset now supports negative values

## Version 1.5.3
### Fix:
- Token now derives Default trait

## Version 1.5.2
### Fix:
- Message now does not consumes itself when it's being displayed
- Removed `warn*` and `info*` macros as we don't see any reason to use them at this point

## Version 1.5.1
### Feature:
- Added `error_at`, `warn_at`, `info_at` macros

## Version 1.5.0
### Feature:
- _Breaking change:_ All new Failing API
- Syntax Result now returns Failing enum
- Errors are now encouraged to be propagated back to the root of the AST.
- Added `context` macro to support better developer experience

## Version 1.4.0
### Feature:
- Tracebacks

### Fix:
- Terminal colors (support for non-truecolor consoles)
- Logger now prints errors to STDERR

## Version 1.3.1
### Fix:
- Multiline regions wouldn't parse

## Version 1.3.0
### Feature:
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
