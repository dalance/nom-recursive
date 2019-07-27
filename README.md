# nom-recursive
An extension of [nom](https://github.com/Geal/nom) to handle left recursion.

[![Build Status](https://dev.azure.com/dalance/procs/_apis/build/status/dalance.nom-recursive?branchName=master)](https://dev.azure.com/dalance/nom-recursive/_build/latest?definitionId=1&branchName=master)
[![Crates.io](https://img.shields.io/crates/v/nom-recursive.svg)](https://crates.io/crates/nom-recursive)
[![Docs.rs](https://docs.rs/nom-recursive/badge.svg)](https://docs.rs/nom-recursive)

## Requirement

nom must be 5.0.0 or later.
nom-recursive can be applied to function-style parser only.

The input type of nom parser must implement `HasRecursiveInfo` trait.
Therefore `&str` and `&[u8]` can't be used.
You can define a wrapper type of `&str` or `&[u8]` and implement `HasRecursiveInfo`.

Alternatively you can use `nom_locate::LocatedSpanEx<T, RecursiveInfo>`.
This implements `HasRecursiveInfo` in this crate.

## Usage

```Cargo.toml
[dependencies]
nom-recursive = "0.1.0"
```

## Example

```rust
use nom::branch::*;
use nom::character::complete::*;
use nom::IResult;
use nom_locate::LocatedSpanEx;
use nom_recursive::{recursive_parser, RecursiveInfo};

// Input type must implement trait HasRecursiveInfo
// nom_locate::LocatedSpanEx<T, RecursiveInfo> implements it.
type Span<'a> = LocatedSpanEx<&'a str, RecursiveInfo>;

pub fn expr(s: Span) -> IResult<Span, String> {
    alt((expr_binary, term))(s)
}

// Apply recursive_parser by custom attribute
#[recursive_parser]
pub fn expr_binary(s: Span) -> IResult<Span, String> {
    let (s, x) = expr(s)?;
    let (s, y) = char('+')(s)?;
    let (s, z) = expr(s)?;
    let ret = format!("{}{}{}", x, y, z);
    Ok((s, ret))
}

pub fn term(s: Span) -> IResult<Span, String> {
    let (s, x) = char('1')(s)?;
    Ok((s, x.to_string()))
}

fn main() {
    let ret = expr(LocatedSpanEx::new_extra("1+1", RecursiveInfo::new()));
    println!("{:?}", ret.unwrap().1);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
