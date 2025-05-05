<a href="https://rust-lambda.com/" alt="The book cover of the book Crafting Lambda Functions in Rust" align="right"><img align="right" src="https://rust-lambda.com/_astro/cover-light.aCUulJ1e_10Kasa.webp" style="float: right; margin-left: 1em; z-index:1000" width="200"/></a>

> [!TIP]
> Do you want to learn how to write efficient and cost-effective AWS Lambda functions using Rust? Check out my book [Crafting Lambda Functions in Rust](https://rust-lambda.com/)
>
> 
> 
> 

```
   _      _
  (<      >)
   `O,99,O`
  //-\__/-\\
```


# Base36 Encoder

This crate provides a simple function to convert an integer to its string representation using a base-36 encoding scheme. It is a solution to the **Rust Challenge** from the [Rust Bytes newsletter of **May 04, 2025**](https://weeklyrust.substack.com/p/bevy-got-a-glow-up).




## Overview

Base-36 encoding is a positional numeral system that uses 36 characters: the digits `0-9` and the letters `a-z`. This encoding is often used in scenarios where compact and human-readable representations of numbers are needed, such as URL shorteners, unique identifiers, or compact storage of numeric data.

This crate provides a simple implementation of base-36 encoding in Rust. While it may not be the most optimal or efficient solution, it serves as a great starting point for understanding how to implement a base-36 encoding scheme in Rust.

## Features

- Converts integers (both positive and negative) to their base-36 string representation.
- Handles edge cases such as zero and negative numbers.
- Simple and easy-to-read implementation.

## Example Usage

```rust
use base36_encoder::to_base36;

fn main() {
    let number = 123456789;
    let base36 = to_base36(number);
    println!("Base-36 representation of {} is {}", number, base36);
    // Output: Base-36 representation of 123456789 is 21i3v9
}
```

## Installation

This crate is not published on crates.io.
What you probably want to do instead of using this is to use the [`num`](https://crates.io/crates/num) crate, which provides a more comprehensive set of numeric utilities, including base-36 encoding:

```rust
use num::Integer;
use num::bigint::ToBigInt;

fn to_base36(n: u64) -> String {
    use num::bigint::BigUint;
    use num::traits::ToPrimitive;

    let bigint = BigUint::from(n);
    bigint.to_str_radix(36)
}
```

If you really want to use it, you can copy paste the code into your project and add it as a local dependency in your `Cargo.toml` file:

```toml
[dependencies]
base36_encoder = { path = "../base36_encoder" }
```

## Limitations

- This implementation is not optimized for performance or memory usage. If you need a highly efficient solution for production use, consider exploring other libraries or optimizing this implementation.
- The function currently supports only integers. If you need support for floating-point numbers or other data types, additional work will be required.

## Motivation

This crate was created as a solution to the **Rust Challenge** from the [Rust Bytes newsletter](https://weeklyrust.substack.com/p/bevy-got-a-glow-up) of **May 04, 2025**. The challenge was a fun exercise to practice Rust programming and explore how to implement a base-36 encoding scheme.

## Contributing

Contributions are welcome! If you have suggestions for improving the implementation or adding new features, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Happy coding! 🚀
