//! **Embedded license information from [SPDX](https://spdx.org).**
//!
//! Use the licenses directly.
//!
//! ```
//! use license::License;
//! use license::licenses::Bsd3Clause;
//!
//! let bsd3 = Bsd3Clause;
//! assert!(bsd3.is_osi_approved());
//! assert_eq!(bsd3.name(), r#"BSD 3-Clause "New" or "Revised" License"#);
//! ```
//!
//! Get the license by parsing the license id.
//!
//! ```
//! use license::License;
//!
//! let apache2: &dyn License = "Apache-2.0".parse().unwrap();
//! assert_eq!(apache2.name(), "Apache License 2.0");
//! ```
//!
//! License exceptions are also supported.
//!
//! ```
//! use license::Exception;
//!
//! let gcc: &dyn Exception = "GCC-exception-3.1".parse().unwrap();
//! assert_eq!(gcc.name(), "GCC Runtime Library exception 3.1");
//! ```
//!
//! [Serde](https://crates.io/crates/serde) is supported with the `serde` feature.

#![no_std]
#![doc(html_root_url = "https://docs.rs/license")]
#![deny(missing_docs, unsafe_code)]

#[cfg(feature = "serde")]
mod serde;

use core::fmt::{self, Debug, Display, Formatter};
use core::str::FromStr;

/// All supported licenses.
pub mod licenses {
    include!(concat!(env!("OUT_DIR"), "/licenses.rs"));
}

/// All supported exceptions.
pub mod exceptions {
    include!(concat!(env!("OUT_DIR"), "/exceptions.rs"));
}

/// Base functionality for all licenses.
pub trait License {
    /// The identifier of the license.
    ///
    /// Corresponds to the *Identifier* column from [spdx.org/licenses](https://spdx.org/licenses/).
    fn id(&self) -> &'static str;

    /// The name of the license.
    ///
    /// Corresponds to the *Full name* column from [spdx.org/licenses](https://spdx.org/licenses/).
    fn name(&self) -> &'static str;

    /// The license text.
    fn text(&self) -> &'static str;

    /// The standard license header.
    fn header(&self) -> Option<&'static str>;

    /// Says if the license is OSI approved.
    ///
    /// Corresponds to the *OSI Approved?* column from [spdx.org/licenses](https://spdx.org/licenses/).
    fn is_osi_approved(&self) -> bool;

    /// Says if the license is FSF Libre.
    ///
    /// Corresponds to the *FSF Free/Libre?* column from [spdx.org/licenses](https://spdx.org/licenses/).
    fn is_fsf_libre(&self) -> bool;

    /// Says if the license is deprecated.
    fn is_deprecated(&self) -> bool;

    /// The license comments.
    fn comments(&self) -> Option<&'static str>;

    /// Relevant sources.
    fn see_also(&self) -> &'static [&'static str];
}

/// Base functionality for all license exceptions.
pub trait Exception {
    /// The identifier of the exceptions.
    fn id(&self) -> &'static str;

    /// The name of the exception.
    fn name(&self) -> &'static str;

    /// The exception text.
    fn text(&self) -> &'static str;

    /// Says if the exception is deprecated.
    fn is_deprecated(&self) -> bool;

    /// The exception comments.
    fn comments(&self) -> Option<&'static str>;

    /// Relevant sources.
    fn see_also(&self) -> &'static [&'static str];
}

impl Display for dyn License {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self.name(), f)
    }
}

impl Display for dyn Exception {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self.name(), f)
    }
}

impl Debug for dyn License {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self.id(), f)
    }
}

impl Debug for dyn Exception {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self.id(), f)
    }
}

impl FromStr for &dyn License {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        licenses::parse_id(s).ok_or(ParseError(()))
    }
}

impl FromStr for &dyn Exception {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        exceptions::parse_id(s).ok_or(ParseError(()))
    }
}

/// Error returned when parsing license and exception ids.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError(());

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt("SPDX id not found", f)
    }
}
