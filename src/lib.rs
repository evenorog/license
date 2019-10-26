//! Provides license information from [spdx.org](https://spdx.org).

#![doc(html_root_url = "https://docs.rs/license/0.8.0")]
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

#[allow(bad_style)]
mod licenses;

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

/// Extension trait for licenses.
pub trait LicenseExt: License {}

impl LicenseExt for AGPL_3_0_only {}

impl LicenseExt for Apache_2_0 {}

impl LicenseExt for CC0_1_0 {}

impl LicenseExt for GPL_3_0_only {}

impl LicenseExt for LGPL_3_0_only {}

impl LicenseExt for MIT {}

impl LicenseExt for MPL_2_0 {}

impl LicenseExt for Unlicense {}

/// Returns a license based on the license text.
#[inline]
pub fn text(text: &str) -> Option<&'static dyn LicenseExt> {
    let v2 = text.contains("Version 2.0");
    let v3 = text.contains("Version 3");
    if text.contains("MIT License") {
        Some(&MIT)
    } else if v2 && text.contains("Apache License") {
        Some(&Apache_2_0)
    } else if v3 && text.contains("GNU GENERAL PUBLIC LICENSE") {
        Some(&GPL_3_0_only)
    } else if v2 && text.contains("Mozilla Public License") {
        Some(&MPL_2_0)
    } else if text
        .contains("This is free and unencumbered software released into the public domain.")
    {
        Some(&Unlicense)
    } else if v3 && text.contains("GNU LESSER GENERAL PUBLIC LICENSE") {
        Some(&LGPL_3_0_only)
    } else if v3 && text.contains("GNU AFFERO GENERAL PUBLIC LICENSE") {
        Some(&AGPL_3_0_only)
    } else if text.contains("CC0 1.0 Universal") {
        Some(&CC0_1_0)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn text_agpl3() {
        let agpl3 = text(AGPL_3_0_only.text()).unwrap();
        assert_eq!(agpl3.text(), AGPL_3_0_only.text());
    }

    #[test]
    fn text_apache2() {
        let apache2 = text(Apache_2_0.text()).unwrap();
        assert_eq!(apache2.text(), Apache_2_0.text());
    }

    #[test]
    fn text_cc01() {
        let cc01 = text(CC0_1_0.text()).unwrap();
        assert_eq!(cc01.text(), CC0_1_0.text());
    }

    #[test]
    fn text_gpl3() {
        let gpl3 = text(GPL_3_0_only.text()).unwrap();
        assert_eq!(gpl3.text(), GPL_3_0_only.text());
    }

    #[test]
    fn text_lgpl3() {
        let lgpl3 = text(LGPL_3_0_only.text()).unwrap();
        assert_eq!(lgpl3.text(), LGPL_3_0_only.text());
    }

    #[test]
    fn text_mit() {
        let mit = text(MIT.text()).unwrap();
        assert_eq!(mit.text(), MIT.text());
    }

    #[test]
    fn text_mpl2() {
        let mpl2 = text(MPL_2_0.text()).unwrap();
        assert_eq!(mpl2.text(), MPL_2_0.text());
    }

    #[test]
    fn text_unlicense() {
        let unlicense = text(Unlicense.text()).unwrap();
        assert_eq!(unlicense.text(), Unlicense.text());
    }
}
