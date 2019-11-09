![license](https://raw.githubusercontent.com/evenorog/license/master/license.svg?sanitize=true)

[![Travis](https://travis-ci.com/evenorog/license.svg?branch=master)](https://travis-ci.com/evenorog/license)
[![Crates.io](https://img.shields.io/crates/v/license.svg)](https://crates.io/crates/license)
[![Docs](https://docs.rs/license/badge.svg)](https://docs.rs/license)

Provides license information from [spdx.org](https://spdx.org).

```rust
let apache2 = license::from_id("Apache-2.0").unwrap();
assert_eq!(apache2.name(), "Apache License 2.0");
```

The library also extends certain licenses with information about their limitations, conditions, and permission.

```rust
let mit = license::from_id_ext("MIT").unwrap();
let perm = mit.permissions();
assert!(perm.private_use() && perm.commercial_use());
```

### License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
