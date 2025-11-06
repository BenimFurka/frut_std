# Frut Standard Library

[![Crates.io](https://img.shields.io/crates/v/frut_std.svg)](https://crates.io/crates/frut_std)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/BenimFurka/frut/blob/main/LICENSE)

Standard library for the Frut programming language. Provides built-in functions, native modules, and primitive-method helpers for use with the `frut_lib` analyzer and runtime.

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
frut_std = "0.0.4"
```
Or use this comand:
```bash
cargo add frut_std
```

## Features
- **Built-ins**: `print(string): void`, `println(string): void`, `input(string): string`.
- **Native modules**:
  - `std/io`: re-exports `print`, `println`, `input`.
  - `std/math`: `abs_i(int): int`, `abs_d(double): double`, `min_i(int,int): int`, `min_d(double,double): double`, `max_i(int,int): int`, `max_d(double,double): double`.
- **Primitive methods (runtime-dispatched)**:
  - `string.len(): int`, `string.contains(string): bool`
  - `int.abs(): int`, `double.abs(): double`
  - `bool.to_int(): int`

## Umm
Hey, crate is not finished yet, but its working. 

## Quick start

Use with `frut_lib` by predeclaring built-ins for semantic analysis and registering native implementations in the runtime:

```rust
use frut_lib::{semantic::SemanticAnalyzer, value::RuntimeEnvironment};
use frut_std::{predeclare_std_builtins, register_std_builtins, register_native_modules};

fn setup() {
    // Semantic predeclare
    let mut analyzer = SemanticAnalyzer::default();
    predeclare_std_builtins(&mut analyzer).unwrap();

    // Runtime registration
    let mut env = RuntimeEnvironment::default();
    register_std_builtins(&mut env);

    // Optionally register specific native modules (or None to import all symbols)
    register_native_modules(
        &mut env,
        &[
            ("std/io".to_string(), None),
            ("std/math".to_string(), None),
        ],
    );
}
```

In Frut code you can import and use modules:

```frut
import std.io;
import std.math;

println("Hello, Frut!");
println("min: " + min_i(3, 5));
```

Try also `frut_interp` crate to run this code, that crate uses `frut_std` crate.

## Documentation
See the Frut wiki for the standard library overview and usage:
- https://github.com/BenimFurka/frut/wiki/Standard-Library

## License
Licensed under the Apache License, Version 2.0 [LICENSE](LICENSE).
