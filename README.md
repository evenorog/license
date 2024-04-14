# license

**Embedded license information from [SPDX](https://spdx.org).**

[![Rust](https://github.com/evenorog/license/actions/workflows/rust.yml/badge.svg)](https://github.com/evenorog/license/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/license.svg)](https://crates.io/crates/license)
[![Docs](https://docs.rs/license/badge.svg)](https://docs.rs/license)

Use the licenses directly.

```rust
use license::License;
use license::licenses::Bsd3Clause;

let bsd3 = Bsd3Clause;
assert!(bsd3.is_osi_approved());
assert_eq!(bsd3.name(), r#"BSD 3-Clause "New" or "Revised" License"#);
```

Get the license by parsing the license id.

```rust
use license::License;

let apache2: & dyn License = "Apache-2.0".parse().unwrap();
assert_eq!(apache2.name(), "Apache License 2.0");
```

License exceptions are also supported.

```rust
use license::Exception;

let gcc: & dyn Exception = "GCC-exception-3.1".parse().unwrap();
assert_eq!(gcc.name(), "GCC Runtime Library exception 3.1");
```

[Serde](https://crates.io/crates/serde) is supported with the `serde` feature.

### License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
