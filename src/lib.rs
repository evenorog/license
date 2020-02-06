//! Provides embedded license information from [SPDX](https://spdx.org).
//!
//! ```
//! let apache2 = license::from_id("Apache-2.0").unwrap();
//! assert_eq!(apache2.name(), "Apache License 2.0");
//! ```
//!
//! The library also extends certain licenses with information about their limitations, conditions, and permission.
//!
//! ```
//! let mit = license::from_id_ext("MIT").unwrap();
//! let perm = mit.permissions();
//! assert!(perm.private_use() && perm.commercial_use());
//! ```
//!
//! License exceptions are also supported.
//!
//! ```
//! let gcc = license::from_id_exception("GCC-exception-3.1").unwrap();
//! assert_eq!(gcc.name(), "GCC Runtime Library exception 3.1");
//! ```

#![no_std]
#![doc(html_root_url = "https://docs.rs/license")]
#![deny(missing_docs)]

#[allow(bad_style)]
mod exceptions;
mod ext;
#[allow(bad_style)]
mod licenses;

pub use exceptions::*;
pub use ext::*;
pub use licenses::*;

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

/// Extension trait for supported licenses.
pub trait LicenseExt: License {
    /// The permissions of the license.
    fn permissions(&self) -> Permissions;

    /// The conditions of the license.
    fn conditions(&self) -> Conditions;

    /// The limitations of the license.
    fn limitations(&self) -> Limitations;
}

/// Base functionality for all license exceptions.
pub trait LicenseException {
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
