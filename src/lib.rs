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

use core::fmt::{self, Display, Formatter};
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
pub trait LicenseExt: License {
    /// The permissions of the license.
    fn permissions(&self) -> Permissions;

    /// The conditions of the license.
    fn conditions(&self) -> Conditions;

    /// The limitations of the license.
    fn limitations(&self) -> Limitations;
}

impl LicenseExt for AGPL_3_0_only {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: true,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: true,
            document_changes: true,
            license_and_copyright_notice: true,
            network_use_is_distribution: true,
            same_license: true,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: false,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for Apache_2_0 {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: true,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: false,
            document_changes: true,
            license_and_copyright_notice: true,
            network_use_is_distribution: false,
            same_license: false,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: true,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for CC0_1_0 {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: false,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: false,
            document_changes: false,
            license_and_copyright_notice: false,
            network_use_is_distribution: false,
            same_license: false,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: true,
            no_warranty: true,
            no_patent_rights: true,
        }
    }
}

impl LicenseExt for GPL_3_0_only {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: true,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: true,
            document_changes: true,
            license_and_copyright_notice: true,
            network_use_is_distribution: false,
            same_license: true,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: false,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for LGPL_3_0_only {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: true,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: true,
            document_changes: true,
            license_and_copyright_notice: true,
            network_use_is_distribution: false,
            same_license: true,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: false,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for MIT {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: false,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: false,
            document_changes: false,
            license_and_copyright_notice: true,
            network_use_is_distribution: false,
            same_license: false,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: false,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for MPL_2_0 {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: true,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: true,
            document_changes: false,
            license_and_copyright_notice: true,
            network_use_is_distribution: false,
            same_license: true,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: true,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

impl LicenseExt for Unlicense {
    #[inline]
    fn permissions(&self) -> Permissions {
        Permissions {
            commercial_use: true,
            distribution: true,
            modification: true,
            patent_rights: false,
            private_use: true,
        }
    }

    #[inline]
    fn conditions(&self) -> Conditions {
        Conditions {
            disclose_sources: false,
            document_changes: false,
            license_and_copyright_notice: false,
            network_use_is_distribution: false,
            same_license: false,
        }
    }

    #[inline]
    fn limitations(&self) -> Limitations {
        Limitations {
            no_liability: true,
            no_trademark_rights: false,
            no_warranty: true,
            no_patent_rights: false,
        }
    }
}

/// The permissions of the license.
#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Permissions {
    /// May be used for commercial purposes.
    pub commercial_use: bool,
    /// May be distributed.
    pub distribution: bool,
    /// May be modified.
    pub modification: bool,
    /// Provides an express grant of patent rights from contributors.
    pub patent_rights: bool,
    /// May be used for private purposes.
    pub private_use: bool,
}

impl Display for Permissions {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.commercial_use {
            f.write_str("- May be used for commercial purposes.\n")?;
        }
        if self.distribution {
            f.write_str("- May be distributed.\n")?;
        }
        if self.modification {
            f.write_str("- May be modified.\n")?;
        }
        if self.patent_rights {
            f.write_str("- Provides an express grant of patent rights from contributors.\n")?;
        }
        if self.private_use {
            f.write_str("- May be used for private purposes.\n")?;
        }
        Ok(())
    }
}

/// The conditions of the license.
#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Conditions {
    /// Source code must be made available when the software is distributed.
    pub disclose_sources: bool,
    /// Changes made to the code must be documented.
    pub document_changes: bool,
    /// The license and copyright notice must be included with the software.
    pub license_and_copyright_notice: bool,
    /// Users who interact with the software via network are
    /// given the right to receive a copy of the source code.
    pub network_use_is_distribution: bool,
    /// Modifications must be released under the same license.
    pub same_license: bool,
}

impl Display for Conditions {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.disclose_sources {
            f.write_str(
                "- Source code must be made available when the software is distributed.\n",
            )?;
        }
        if self.document_changes {
            f.write_str("- Changes made to the code must be documented.\n")?;
        }
        if self.license_and_copyright_notice {
            f.write_str(
                "- The license and copyright notice must be included with the software.\n",
            )?;
        }
        if self.network_use_is_distribution {
            f.write_str("- Users who interact with the software via network are given the right to receive a copy of the source code.\n")?;
        }
        if self.same_license {
            f.write_str("- Modifications must be released under the same license.\n")?;
        }
        Ok(())
    }
}

/// The limitations of the license.
#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Limitations {
    /// Includes a limitation of liability.
    pub no_liability: bool,
    /// Does not grant trademark rights.
    pub no_trademark_rights: bool,
    /// Does not provide any warranty.
    pub no_warranty: bool,
    /// Does not provide any rights in the patents of contributors.
    pub no_patent_rights: bool,
}

impl Display for Limitations {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.no_liability {
            f.write_str("- Includes a limitation of liability.\n")?;
        }
        if self.no_trademark_rights {
            f.write_str("- Does not grant trademark rights.\n")?;
        }
        if self.no_warranty {
            f.write_str("- Does not provide any warranty.\n")?;
        }
        if self.no_patent_rights {
            f.write_str("- Does not provide any rights in the patents of contributors.\n")?;
        }
        Ok(())
    }
}

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
