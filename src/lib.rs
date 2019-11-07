//! Provides license information from [spdx.org](https://spdx.org).
//!
//! The library also extends certain licenses with information about their limitations, conditions, and permission.

#![doc(html_root_url = "https://docs.rs/license/0.8.1")]
#![no_std]
#![deny(
    bad_style,
    bare_trait_objects,
    missing_debug_implementations,
    missing_docs,
    unused_import_braces,
    unused_qualifications,
    unsafe_code,
    unstable_features
)]

mod ext;
#[allow(bad_style)]
mod licenses;

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
