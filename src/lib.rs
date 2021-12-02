//! Provides embedded license information from [SPDX](https://spdx.org).
//!
//! ```
//! # use license::License;
//! let apache2 = "Apache-2.0".parse::<&dyn License>().unwrap();
//! assert_eq!(apache2.name(), "Apache License 2.0");
//! ```
//!
//! License exceptions are also supported.
//!
//! ```
//! # use license::Exception;
//! let gcc = "GCC-exception-3.1".parse::<&dyn Exception>().unwrap();
//! assert_eq!(gcc.name(), "GCC Runtime Library exception 3.1");
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/license")]
#![allow(bad_style)]
#![deny(missing_docs)]

use core::fmt;
use core::fmt::Formatter;
use core::str::FromStr;

include!(concat!(env!("OUT_DIR"), "/licenses.rs"));
include!(concat!(env!("OUT_DIR"), "/exceptions.rs"));

/// Base functionality for all licenses.
pub trait License {
    /// The name of the license.
    ///
    /// Corresponds to the *Full name* column from [spdx.org/licenses](https://spdx.org/licenses/).
    fn name(&self) -> &'static str;

    /// The identifier of the license.
    ///
    /// Corresponds to the *Identifier* column from [spdx.org/licenses](https://spdx.org/licenses/).
    fn id(&self) -> &'static str;

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

    /// Relevant sources.
    fn see_also(&self) -> &'static [&'static str];
}

/// Base functionality for all license exceptions.
pub trait Exception {
    /// The name of the exception.
    fn name(&self) -> &'static str;

    /// The identifier of the exceptions.
    fn id(&self) -> &'static str;

    /// The exception text.
    fn text(&self) -> &'static str;

    /// Says if the exception is deprecated.
    fn is_deprecated(&self) -> bool;

    /// The exception comments.
    fn comments(&self) -> Option<&'static str>;

    /// Relevant sources.
    fn see_also(&self) -> &'static [&'static str];
}

impl FromStr for &'static dyn License {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_license_id(s).ok_or(ParseError(()))
    }
}

impl FromStr for &'static dyn Exception {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_exception_id(s).ok_or(ParseError(()))
    }
}

/// Error returned when parsing license and exception ids.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError(());

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        "provided id does not match".fmt(f)
    }
}
